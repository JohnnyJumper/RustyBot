-- CreateTable
CREATE TABLE "user" (
    "id" BIGSERIAL NOT NULL,
    "username" VARCHAR NOT NULL,
    "reputation" DOUBLE PRECISION NOT NULL DEFAULT 100.0,
    "discord_user_id" VARCHAR NOT NULL,

    CONSTRAINT "newtable_pk" PRIMARY KEY ("id")
);
