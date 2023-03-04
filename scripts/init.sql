CREATE TABLE IF NOT EXISTS "public"."users" (
  "id" uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  "email" VARCHAR(255) NOT NULL,
  "password" VARCHAR(255) NOT NULL,
  "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

INSERT INTO
  "public"."users" (email, password)
VALUES
  ('user1@example.com', 'password1'),
  ('user2@example.com', 'password2'),
  ('user3@example.com', 'password3');