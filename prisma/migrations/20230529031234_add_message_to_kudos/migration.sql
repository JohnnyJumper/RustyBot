/*
  Warnings:

  - Added the required column `message` to the `kudos` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "kudos" ADD COLUMN  "message" TEXT NOT NULL;
