//! Email sending trait and shared templates.

use async_trait::async_trait;

/// Abstraction over how outbound email is actually delivered.
#[async_trait]
pub trait EmailSender: Send + Sync {
    async fn send(&self, to: &str, subject: &str, html_body: &str) -> anyhow::Result<()>;
}

/// Render the workspace invitation email.
pub fn render_invite_email(
    workspace_name: &str,
    inviter_name: &str,
    role: &str,
    invite_url: &str,
) -> (String, String) {
    let subject = format!("{} invited you to join \"{}\" on Shortas", inviter_name, workspace_name);
    let html = format!(
        r#"<div style="font-family: sans-serif; max-width: 480px;">
            <p>{inviter_name} invited you to join the <strong>{workspace_name}</strong> workspace on Shortas as <strong>{role}</strong>.</p>
            <p><a href="{invite_url}" style="display:inline-block;padding:10px 20px;background:#4f46e5;color:#fff;border-radius:6px;text-decoration:none;">Accept invitation</a></p>
            <p style="color:#666;font-size:12px;">This invitation expires in 7 days. If you weren't expecting this, you can ignore this email.</p>
        </div>"#,
        inviter_name = inviter_name,
        workspace_name = workspace_name,
        role = role,
        invite_url = invite_url,
    );
    (subject, html)
}
