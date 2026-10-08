//! Misskey の `api.json` が、サーバが実際に返す内容より多くを保証している箇所。
//! 補正は、該当するスキーマを持つすべての spec に、型を解決する前に適用する。
//! こうすることで、`specs/` のファイルを上流が生成したままに保つ。

use indexmap::IndexMap;

use crate::spec::{AdditionalProperties, Schema, SchemaType, Spec};

pub fn apply(spec: &mut Spec) {
    let schemas = &mut spec.components.schemas;

    // この係数は倍率である。
    // Web クライアントの初期設定ウィザードは、0.3・0.7・1.25・1.5 といった値を保存する。
    if let Some(factor) = property(schemas, "RolePolicies", &["rateLimitFactor"]) {
        factor.schema_type = Some(SchemaType::One("number".to_owned()));
    }

    // 保存されているベースロールから、既定値を補わずに取り出される（`MetaEntityService`）。
    // そのため、管理者がベースロールを一度保存するまでは、どちらも欠ける。
    optional(
        property(schemas, "MetaDetailedOnly", &["features"]),
        &["localTimeline", "globalTimeline"],
    );

    // この列の初期値は `{}` で、初期設定ウィザードは `entrancePageStyle` だけを保存する。
    optional(
        property(schemas, "MetaClientOptions", &[]),
        &[
            "entrancePageStyle",
            "showTimelineForVisitor",
            "showActivitiesForVisitor",
        ],
    );

    // `i` は、未読のお知らせを保存されている形のまま返す（`UserEntityService`）。
    // ほかの箇所で `AnnouncementEntityService` が足す `forYou` は付かない。
    optional(property(schemas, "Announcement", &[]), &["forYou"]);

    // BullMQ のジョブの値をそのまま返す（`QueueService` の `packJobData`）。
    // `progress` は 0 から始まり、`returnValue` は処理が値を返すまで null で、どちらも object とは限らない。
    // `failedReason` は、ジョブが失敗するまで無い。
    for name in ["progress", "returnValue"] {
        if let Some(value) = property(schemas, "QueueJob", &[name]) {
            value.schema_type = None;
        }
    }
    optional(property(schemas, "QueueJob", &[]), &["failedReason"]);

    // ロールのポリシーの上書きは、値をそのポリシーの型のまま返す。
    // 上書きの無いポリシーも既定値で埋めるので（`RoleEntityService`）、小数（`rateLimitFactor`）、
    // 文字列（`chatAvailability`）、配列（`uploadableFileTypes`）も入り、整数か真偽値とは限らない。
    if let Some(value) = role_policy_value(schemas) {
        value.one_of = None;
    }
}

fn property<'a>(
    schemas: &'a mut IndexMap<String, Schema>,
    component: &str,
    path: &[&str],
) -> Option<&'a mut Schema> {
    path.iter()
        .try_fold(schemas.get_mut(component)?, |schema, name| {
            schema.properties.as_mut()?.get_mut(*name)
        })
}

/// `Role` の `policies` の各要素にある `value`。
/// `Role` は allOf で、`policies` はマップなので、[`property`] ではたどれない。
fn role_policy_value(schemas: &mut IndexMap<String, Schema>) -> Option<&mut Schema> {
    let policies = schemas
        .get_mut("Role")?
        .all_of
        .as_mut()?
        .iter_mut()
        .find_map(|member| member.properties.as_mut()?.get_mut("policies"))?;
    let Some(AdditionalProperties::Schema(entry)) = policies.additional_properties.as_mut() else {
        return None;
    };
    entry
        .any_of
        .as_mut()?
        .iter_mut()
        .find_map(|member| member.properties.as_mut()?.get_mut("value"))
}

fn optional(schema: Option<&mut Schema>, names: &[&str]) {
    if let Some(required) = schema.and_then(|schema| schema.required.as_mut()) {
        required.retain(|name| !names.contains(&name.as_str()));
    }
}
