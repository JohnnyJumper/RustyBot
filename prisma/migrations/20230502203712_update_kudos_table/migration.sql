/*
  Warnings:

  - A unique constraint covering the columns `[id,discord_user_id]` on the table `user` will be added. If there are existing duplicate values, this will fail.
  - Added the required column `from_discord_id` to the `kudos` table without a default value. This is not possible if the table is not empty.
  - Added the required column `to_discord_id` to the `kudos` table without a default value. This is not possible if the table is not empty.
  - Added the required column `to_user_id` to the `kudos` table without a default value. This is not possible if the table is not empty.

*/
-- DropForeignKey
ALTER TABLE "kudos" DROP CONSTRAINT "kudos_from_user_id_fkey";

-- AlterTable
ALTER TABLE "kudos" ADD COLUMN     "from_discord_id" TEXT NOT NULL,
ADD COLUMN     "to_discord_id" TEXT NOT NULL,
ADD COLUMN     "to_user_id" BIGINT NOT NULL;

-- CreateIndex
CREATE UNIQUE INDEX "user_id_discord_user_id_key" ON "user"("id", "discord_user_id");

-- AddForeignKey
ALTER TABLE "kudos" ADD CONSTRAINT "kudos_from_user_id_from_discord_id_fkey" FOREIGN KEY ("from_user_id", "from_discord_id") REFERENCES "user"("id", "discord_user_id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "kudos" ADD CONSTRAINT "kudos_to_user_id_to_discord_id_fkey" FOREIGN KEY ("to_user_id", "to_discord_id") REFERENCES "user"("id", "discord_user_id") ON DELETE RESTRICT ON UPDATE CASCADE;
