-- CreateTable
CREATE TABLE "kudos" (
    "id" BIGSERIAL NOT NULL,
    "from_user_id" BIGINT NOT NULL,
    "timestamp" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "kudos_pk" PRIMARY KEY ("id")
);

-- AddForeignKey
ALTER TABLE "kudos" ADD CONSTRAINT "kudos_from_user_id_fkey" FOREIGN KEY ("from_user_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
