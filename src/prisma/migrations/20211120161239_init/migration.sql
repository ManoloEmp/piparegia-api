-- CreateEnum
CREATE TYPE "State" AS ENUM ('CANCELLED', 'PREPARING', 'DISPATCHED', 'FULFILLED', 'ERROR');

-- CreateEnum
CREATE TYPE "Type" AS ENUM ('PIZZA', 'PASTA', 'SOPA', 'BEBIDA', 'INGREDIENTE');

-- CreateEnum
CREATE TYPE "Variety" AS ENUM ('VEGETARIANA', 'CARNE', 'MIXTA', 'LIGHT');

-- CreateEnum
CREATE TYPE "Size" AS ENUM ('SLICE', 'SMALL', 'MEDIUM', 'LARGE', 'EXTRA');

-- CreateTable
CREATE TABLE "products" (
    "id" TEXT NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "name" JSONB NOT NULL,
    "description" JSONB,
    "image" JSONB,
    "brand_id" TEXT,
    "supplier_id" TEXT,
    "item_type" "Type" NOT NULL DEFAULT E'PIZZA',
    "variety" "Variety" NOT NULL DEFAULT E'VEGETARIANA',
    "ingredients" JSONB,
    "recipe" JSONB,
    "nutritional_info" JSONB,
    "tags" JSONB,
    "performance" JSONB,
    "category_id" TEXT,
    "published" BOOLEAN NOT NULL DEFAULT false,

    PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "brands" (
    "id" TEXT NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "supplier_id" TEXT,
    "name" JSONB NOT NULL,
    "image" JSONB,
    "markup" DOUBLE PRECISION,
    "published" BOOLEAN NOT NULL DEFAULT false,

    PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "suppliers" (
    "id" TEXT NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "name" JSONB NOT NULL,
    "image" JSONB,
    "published" BOOLEAN NOT NULL DEFAULT false,

    PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "variants" (
    "id" TEXT NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "price" DOUBLE PRECISION NOT NULL,
    "weight" TEXT NOT NULL,
    "size" "Size" NOT NULL DEFAULT E'MEDIUM',
    "performance" JSONB,
    "product_id" TEXT NOT NULL,
    "inventory" INTEGER,
    "published" BOOLEAN NOT NULL DEFAULT false,

    PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "categories" (
    "id" TEXT NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "name" JSONB NOT NULL,
    "description" JSONB,
    "roll" JSONB,
    "image" JSONB,
    "tags" JSONB,
    "performance" JSONB,
    "complexity" JSONB,
    "manager_id" TEXT,
    "super_id" TEXT,
    "published" BOOLEAN NOT NULL DEFAULT false,

    PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "managers" (
    "id" TEXT NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "name" TEXT NOT NULL,
    "image" JSONB,
    "published" BOOLEAN NOT NULL DEFAULT false,

    PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "catalogs" (
    "id" TEXT NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "name" JSONB NOT NULL,
    "description" JSONB,
    "tags" JSONB,
    "image" JSONB,
    "published" BOOLEAN NOT NULL DEFAULT false,

    PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "_CatalogToProduct" (
    "A" TEXT NOT NULL,
    "B" TEXT NOT NULL
);

-- CreateTable
CREATE TABLE "_CatalogToVariant" (
    "A" TEXT NOT NULL,
    "B" TEXT NOT NULL
);

-- CreateIndex
CREATE UNIQUE INDEX "_CatalogToProduct_AB_unique" ON "_CatalogToProduct"("A", "B");

-- CreateIndex
CREATE INDEX "_CatalogToProduct_B_index" ON "_CatalogToProduct"("B");

-- CreateIndex
CREATE UNIQUE INDEX "_CatalogToVariant_AB_unique" ON "_CatalogToVariant"("A", "B");

-- CreateIndex
CREATE INDEX "_CatalogToVariant_B_index" ON "_CatalogToVariant"("B");

-- AddForeignKey
ALTER TABLE "products" ADD FOREIGN KEY ("brand_id") REFERENCES "brands"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "products" ADD FOREIGN KEY ("supplier_id") REFERENCES "suppliers"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "products" ADD FOREIGN KEY ("category_id") REFERENCES "categories"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "brands" ADD FOREIGN KEY ("supplier_id") REFERENCES "suppliers"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "variants" ADD FOREIGN KEY ("product_id") REFERENCES "products"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "categories" ADD FOREIGN KEY ("manager_id") REFERENCES "managers"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "categories" ADD FOREIGN KEY ("super_id") REFERENCES "categories"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "_CatalogToProduct" ADD FOREIGN KEY ("A") REFERENCES "catalogs"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "_CatalogToProduct" ADD FOREIGN KEY ("B") REFERENCES "products"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "_CatalogToVariant" ADD FOREIGN KEY ("A") REFERENCES "catalogs"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "_CatalogToVariant" ADD FOREIGN KEY ("B") REFERENCES "variants"("id") ON DELETE CASCADE ON UPDATE CASCADE;
