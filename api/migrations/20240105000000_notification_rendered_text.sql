-- Rendered display text for in-app notifications, produced by the
-- template-rendering pipeline (notification-worker). Nullable so existing
-- rows and any notification created outside that pipeline stay valid.
ALTER TABLE notifications ADD COLUMN rendered_text TEXT;
