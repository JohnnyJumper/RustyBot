/*
  Warnings:

  - A unique constraint covering the columns `[discord_user_id]` on the table `user` will be added. If there are existing duplicate values, this will fail.

*/
-- CreateIndex
CREATE UNIQUE INDEX "user_discord_user_id_key" ON "user"("discord_user_id");
