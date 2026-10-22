// misskey-js の TypeScript の型定義を、codegen が読む構文木の JSON にして標準出力に出す。
//
//   node codegen/ts-types.ts specs/latest/misskey-js/*.ts
//
// 構文木をそのまま写すだけで、名前の解決や型の意味づけは codegen（Rust）が行う。
// TypeScript の compiler API に触るのは parseSourceFile と、SyntaxKind・ノードの判定関数（is〜）・ノードの型だけに
// 閉じている。typescript 7 では createSourceFile が typescript/sync の API のメソッドに、残りが typescript/ast に移るが、
// 名前とノードのプロパティ名は同じなので、そこだけ差し替えれば済む（7.1 beta で名前が違うのは、
// isPropertySignature が isPropertySignatureDeclaration になる 1 件だけ）。
import { readFileSync } from "node:fs";
import { basename } from "node:path";
import ts from "typescript";

type TypeNode =
  | { kind: "keyword"; name: string }
  | { kind: "literal"; value: string | number | boolean }
  | { kind: "ref"; name: string; args: TypeNode[] }
  | { kind: "object"; members: Member[] }
  | { kind: "union"; types: TypeNode[] }
  | { kind: "intersection"; types: TypeNode[] }
  | { kind: "array"; element: TypeNode }
  | { kind: "indexed"; object: TypeNode; index: TypeNode }
  | { kind: "function"; typeParams: TypeParam[]; params: TypeNode[] }
  | { kind: "typeof"; name: string }
  | { kind: "unsupported"; syntax: string; text: string };
type Member = { name: string; optional: boolean; type: TypeNode; doc: string | null };
type TypeParam = { name: string; constraint: TypeNode | null };
type TypeDecl = { name: string; typeParams: TypeParam[]; type: TypeNode };
type ConstDecl = { name: string; values: string[] | null };
type Import = { name: string; from: string };
type File = { imports: Import[]; reexports: string[]; types: TypeDecl[]; consts: ConstDecl[] };

function parseSourceFile(path: string, text: string): ts.SourceFile {
  return ts.createSourceFile(path, text, ts.ScriptTarget.Latest, true);
}

function convertFile(source: ts.SourceFile): File {
  const file: File = { imports: [], reexports: [], types: [], consts: [] };
  for (const statement of source.statements) {
    if (ts.isImportDeclaration(statement)) {
      const from = (statement.moduleSpecifier as ts.StringLiteral).text;
      const bindings = statement.importClause?.namedBindings;
      if (bindings !== undefined && ts.isNamedImports(bindings)) {
        for (const element of bindings.elements) {
          // 別名の import（`import { A as B }`）には対応していない。B を記録すると import 元の B を引いてしまうので、
          // 記録せずに、名前が無いエラーで止める。
          if (element.propertyName === undefined) {
            file.imports.push({ name: element.name.text, from });
          }
        }
      }
    } else if (ts.isExportDeclaration(statement)) {
      if (statement.exportClause === undefined && statement.moduleSpecifier !== undefined) {
        file.reexports.push((statement.moduleSpecifier as ts.StringLiteral).text);
      }
    } else if (ts.isTypeAliasDeclaration(statement)) {
      file.types.push({
        name: statement.name.text,
        typeParams: (statement.typeParameters ?? []).map((p) => convertTypeParam(p, source)),
        type: convertType(statement.type, source),
      });
    } else if (ts.isInterfaceDeclaration(statement)) {
      file.types.push({
        name: statement.name.text,
        typeParams: [],
        type: unsupported(statement, source),
      });
    } else if (ts.isVariableStatement(statement)) {
      for (const declaration of statement.declarationList.declarations) {
        if (ts.isIdentifier(declaration.name)) {
          file.consts.push({ name: declaration.name.text, values: stringArray(declaration.initializer) });
        }
      }
    }
  }
  return file;
}

/// `['a', 'b'] as const satisfies T` の文字列の配列。それ以外の初期化子は null。
function stringArray(expression: ts.Expression | undefined): string[] | null {
  while (
    expression !== undefined &&
    (ts.isAsExpression(expression) || ts.isSatisfiesExpression(expression) || ts.isParenthesizedExpression(expression))
  ) {
    expression = expression.expression;
  }
  if (expression === undefined || !ts.isArrayLiteralExpression(expression)) {
    return null;
  }
  const values: string[] = [];
  for (const element of expression.elements) {
    if (!ts.isStringLiteral(element)) {
      return null;
    }
    values.push(element.text);
  }
  return values;
}

function convertTypeParam(param: ts.TypeParameterDeclaration, source: ts.SourceFile): TypeParam {
  return {
    name: param.name.text,
    constraint: param.constraint === undefined ? null : convertType(param.constraint, source),
  };
}

function convertType(node: ts.TypeNode, source: ts.SourceFile): TypeNode {
  switch (node.kind) {
    case ts.SyntaxKind.StringKeyword:
      return { kind: "keyword", name: "string" };
    case ts.SyntaxKind.NumberKeyword:
      return { kind: "keyword", name: "number" };
    case ts.SyntaxKind.BooleanKeyword:
      return { kind: "keyword", name: "boolean" };
    case ts.SyntaxKind.AnyKeyword:
      return { kind: "keyword", name: "any" };
    case ts.SyntaxKind.UnknownKeyword:
      return { kind: "keyword", name: "unknown" };
    case ts.SyntaxKind.NeverKeyword:
      return { kind: "keyword", name: "never" };
    case ts.SyntaxKind.LiteralType: {
      const literal = (node as ts.LiteralTypeNode).literal;
      if (ts.isStringLiteral(literal)) {
        return { kind: "literal", value: literal.text };
      }
      if (ts.isNumericLiteral(literal)) {
        return { kind: "literal", value: Number(literal.text) };
      }
      if (literal.kind === ts.SyntaxKind.TrueKeyword) {
        return { kind: "literal", value: true };
      }
      if (literal.kind === ts.SyntaxKind.FalseKeyword) {
        return { kind: "literal", value: false };
      }
      if (literal.kind === ts.SyntaxKind.NullKeyword) {
        return { kind: "keyword", name: "null" };
      }
      return unsupported(node, source);
    }
    case ts.SyntaxKind.TypeReference: {
      const reference = node as ts.TypeReferenceNode;
      return {
        kind: "ref",
        name: entityName(reference.typeName),
        args: (reference.typeArguments ?? []).map((argument) => convertType(argument, source)),
      };
    }
    case ts.SyntaxKind.TypeLiteral: {
      const members: Member[] = [];
      for (const member of (node as ts.TypeLiteralNode).members) {
        if (!ts.isPropertySignature(member) || member.type === undefined) {
          return unsupported(member, source);
        }
        const name = propertyName(member.name);
        if (name === null) {
          return unsupported(member, source);
        }
        members.push({
          name,
          optional: member.questionToken !== undefined,
          type: convertType(member.type, source),
          doc: trailingComment(member, source),
        });
      }
      return { kind: "object", members };
    }
    case ts.SyntaxKind.UnionType:
      return { kind: "union", types: (node as ts.UnionTypeNode).types.map((t) => convertType(t, source)) };
    case ts.SyntaxKind.IntersectionType:
      return {
        kind: "intersection",
        types: (node as ts.IntersectionTypeNode).types.map((t) => convertType(t, source)),
      };
    case ts.SyntaxKind.ArrayType:
      return { kind: "array", element: convertType((node as ts.ArrayTypeNode).elementType, source) };
    case ts.SyntaxKind.IndexedAccessType: {
      const access = node as ts.IndexedAccessTypeNode;
      return {
        kind: "indexed",
        object: convertType(access.objectType, source),
        index: convertType(access.indexType, source),
      };
    }
    case ts.SyntaxKind.ParenthesizedType:
      return convertType((node as ts.ParenthesizedTypeNode).type, source);
    case ts.SyntaxKind.FunctionType: {
      const fn = node as ts.FunctionTypeNode;
      return {
        kind: "function",
        typeParams: (fn.typeParameters ?? []).map((p) => convertTypeParam(p, source)),
        // 任意の引数（`?`）と残余引数（`...`）は、本文の有無や形が変わるので対応していない。
        params: fn.parameters.map((p) =>
          p.type === undefined || p.questionToken !== undefined || p.dotDotDotToken !== undefined
            ? unsupported(p, source)
            : convertType(p.type, source),
        ),
      };
    }
    case ts.SyntaxKind.TypeQuery:
      return { kind: "typeof", name: entityName((node as ts.TypeQueryNode).exprName) };
    default:
      return unsupported(node, source);
  }
}

function entityName(name: ts.EntityName): string {
  return ts.isIdentifier(name) ? name.text : `${entityName(name.left)}.${name.right.text}`;
}

function propertyName(name: ts.PropertyName): string | null {
  if (ts.isIdentifier(name) || ts.isStringLiteral(name) || ts.isNumericLiteral(name)) {
    return name.text;
  }
  return null;
}

/// メンバーと同じ行の、後ろにある `//` コメント。
function trailingComment(node: ts.Node, source: ts.SourceFile): string | null {
  const text = source.text;
  const lineEnd = text.indexOf("\n", node.end);
  const rest = text.slice(node.end, lineEnd === -1 ? text.length : lineEnd);
  const at = rest.indexOf("//");
  if (at === -1 || rest.slice(0, at).trim() !== "") {
    return null;
  }
  return rest.slice(at + 2).trim();
}

function unsupported(node: ts.Node, source: ts.SourceFile): TypeNode {
  return { kind: "unsupported", syntax: ts.SyntaxKind[node.kind], text: node.getText(source) };
}

const files: Record<string, File> = {};
for (const path of process.argv.slice(2)) {
  files[basename(path)] = convertFile(parseSourceFile(path, readFileSync(path, "utf8")));
}
process.stdout.write(JSON.stringify(files));
