//! Mensagens em Espanhol (España)

pub fn help() -> String {
    format!(
        r#"🤖 *BanHammer v{}*

Protege automáticamente los grupos contra:

- Pornografía
- Contenido sexual
- Prostitución
- Juegos de azar
- Spam
- Estafas
- Enlaces maliciosos

Comandos disponibles:

/help - Muestra esta ayuda.

/status - Muestra el estado del bot.

/stats - Muestra estadísticas de moderación del grupo.

/language <pt|en|es> - Cambia el idioma del grupo (solo administradores).

/reload - Recarga moderation.toml sin reiniciar el bot (solo administradores).

/unban <user_id> - Elimina el baneo de un usuario (solo administradores).

/blockdomain <dominio> - Bloquea un dominio al instante (solo administradores).

/trainspam - En respuesta a un mensaje, enseña al clasificador bayesiano que es spam (solo administradores).

/trainham - En respuesta a un mensaje, enseña al clasificador bayesiano que es legítimo (solo administradores)."#,
        env!("CARGO_PKG_VERSION")
    )
}

pub const STATUS: &str = "🟢 BanHammer está en línea y protegiendo este grupo.";

pub const VIOLATION_GENERIC: &str =
    "🚫 Se ha detectado contenido prohibido y se han aplicado las medidas correspondientes.";

pub fn banned(username: &str) -> String {
    format!("🚫 @{username} ha sido expulsado por infringir las normas.")
}

pub fn warned(username: &str, count: i64) -> String {
    format!(
        "⚠️ @{username}, tu mensaje fue eliminado por infringir las normas del grupo. \
         Aviso {count} — las infracciones repetidas resultan en silenciamiento y luego expulsión."
    )
}

pub fn muted(username: &str, minutes: i64) -> String {
    format!(
        "🔇 @{username} ha sido silenciado durante {minutes} minuto(s) por infracciones repetidas."
    )
}

pub fn kicked(username: &str) -> String {
    format!(
        "👢 @{username} fue expulsado del grupo por infracciones repetidas. \
         Puede volver a unirse, pero la próxima infracción resultará en un baneo permanente."
    )
}

pub const LANG_SET: &str = "✅ El idioma del grupo se ha cambiado correctamente a Español.";

pub const LANG_INVALID: &str = "⚠️ Idioma no válido. Utiliza: pt, en o es.";

pub const LANG_NO_PERMISSION: &str =
    "⛔ Solo los administradores pueden cambiar el idioma del grupo.";

pub const STATS_TITLE: &str = "📊 *Estadísticas del grupo*";
pub const STATS_TOTAL: &str = "Violaciones totales";
pub const STATS_24H: &str = "Últimas 24h";
pub const STATS_BY_TYPE: &str = "Por categoría";
pub const STATS_TOP: &str = "Principales infractores";
pub const STATS_EMPTY: &str = "✅ Todavía no hay violaciones registradas en este grupo.";
pub const STATS_BAYES_LEARNED: &str = "Ejemplos enseñados al clasificador bayesiano";

pub const RELOAD_SUCCESS: &str = "✅ Configuración de moderación recargada correctamente.";
pub const RELOAD_ERROR: &str = "⚠️ Error al recargar moderation.toml. Las reglas anteriores siguen activas. Revisa los logs del bot.";
pub const RELOAD_NO_PERMISSION: &str =
    "⚠️ Solo los administradores pueden recargar la configuración.";

pub const UNBAN_NO_PERMISSION: &str = "⚠️ Solo los administradores pueden eliminar baneos.";
pub const UNBAN_INVALID_ID: &str = "⚠️ Uso: `/unban <user_id>` — el ID debe ser numérico.";

pub fn unban_success(user_id: u64) -> String {
    format!("✅ El usuario `{user_id}` ha sido desbaneado.")
}

pub const UNBAN_ERROR: &str = "⚠️ Error al desbanear al usuario. Verifica que el ID sea correcto y que el bot tenga permisos de administrador.";

// /blockdomain <dominio> - bloquea un dominio al instante (solo administradores)

#[allow(non_snake_case)]
pub fn BLOCKDOMAIN_SUCCESS(domain: &str) -> String {
    format!("✅ Dominio `{domain}` bloqueado correctamente.")
}

pub const BLOCKDOMAIN_NO_PERMISSION: &str = "⚠️ Solo los administradores pueden bloquear dominios.";
pub const BLOCKDOMAIN_INVALID: &str =
    "⚠️ Uso: `/blockdomain <dominio>` (ej: /blockdomain spam-site.com).";
pub const BLOCKDOMAIN_ERROR: &str = "⚠️ Error al bloquear el dominio. Revisa los logs del bot.";

pub const TRAIN_NO_PERMISSION: &str =
    "⚠️ Solo los administradores pueden entrenar el clasificador de spam.";
pub const TRAIN_MISSING_REPLY: &str =
    "⚠️ Usa `/trainspam` o `/trainham` en *respuesta* a un mensaje de texto.";
pub const TRAIN_ERROR: &str =
    "⚠️ Error al registrar el ejemplo de entrenamiento. Revisa los logs del bot.";

pub fn train_success(is_spam: bool) -> String {
    if is_spam {
        "✅ Mensaje registrado como *spam*. El clasificador fue reentrenado ahora.".to_string()
    } else {
        "✅ Mensaje registrado como *legítimo*. El clasificador fue reentrenado ahora."
            .to_string()
    }
}
