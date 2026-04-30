CREATE TYPE "public"."digit_type" AS ENUM('Explicit', 'Canonical', 'JCanonical', 'Adjoint', 'Symmetric', 'JSymmetric', 'Shifted');--> statement-breakpoint
CREATE TYPE "public"."job_type" AS ENUM('Walk', 'Decision', 'Classification');--> statement-breakpoint
CREATE TYPE "public"."norm_type" AS ENUM('Infinite', 'L1', 'L2');--> statement-breakpoint
CREATE TYPE "public"."status" AS ENUM('Pending', 'Running', 'Succeeded', 'Failed');--> statement-breakpoint
CREATE TABLE "digits" (
	"id" integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY (sequence name "digits_id_seq" INCREMENT BY 1 MINVALUE 1 MAXVALUE 2147483647 START WITH 1 CACHE 1),
	"elements" integer[] NOT NULL,
	CONSTRAINT "digits_elements_unique" UNIQUE("elements")
);
--> statement-breakpoint
CREATE TABLE "favourites" (
	"user_id" integer NOT NULL,
	"system_id" integer NOT NULL,
	CONSTRAINT "favourites_user_id_system_id_pk" PRIMARY KEY("user_id","system_id")
);
--> statement-breakpoint
CREATE TABLE "jobs" (
	"id" integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY (sequence name "jobs_id_seq" INCREMENT BY 1 MINVALUE 1 MAXVALUE 2147483647 START WITH 1 CACHE 1),
	"user_id" integer NOT NULL,
	"system_id" integer NOT NULL,
	"status" "status" NOT NULL,
	"job_type" "job_type" NOT NULL,
	"norm" "norm_type" NOT NULL,
	"walk_from" integer,
	"output_uri" text,
	"created_at" timestamp (0) with time zone DEFAULT now() NOT NULL,
	"started_at" timestamp (0) with time zone,
	"finished_at" timestamp (0) with time zone,
	"error" text
);
--> statement-breakpoint
CREATE TABLE "systems" (
	"id" integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY (sequence name "systems_id_seq" INCREMENT BY 1 MINVALUE 1 MAXVALUE 2147483647 START WITH 1 CACHE 1),
	"name" text NOT NULL,
	"user_id" integer,
	"dimension" integer NOT NULL,
	"base" integer[] NOT NULL,
	"digit_type" "digit_type" NOT NULL,
	"is_gns" boolean,
	"signature" integer[],
	"last_job" timestamp (0) with time zone,
	"digits" integer[],
	"digit_param" integer
);
--> statement-breakpoint
CREATE TABLE "users" (
	"id" integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY (sequence name "users_id_seq" INCREMENT BY 1 MINVALUE 1 MAXVALUE 2147483647 START WITH 1 CACHE 1),
	"user_name" text NOT NULL,
	"password" text NOT NULL,
	CONSTRAINT "users_user_name_unique" UNIQUE("user_name")
);
--> statement-breakpoint
ALTER TABLE "favourites" ADD CONSTRAINT "favourites_user_id_users_id_fk" FOREIGN KEY ("user_id") REFERENCES "public"."users"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "favourites" ADD CONSTRAINT "favourites_system_id_systems_id_fk" FOREIGN KEY ("system_id") REFERENCES "public"."systems"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "jobs" ADD CONSTRAINT "jobs_user_id_users_id_fk" FOREIGN KEY ("user_id") REFERENCES "public"."users"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "jobs" ADD CONSTRAINT "jobs_system_id_systems_id_fk" FOREIGN KEY ("system_id") REFERENCES "public"."systems"("id") ON DELETE cascade ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "systems" ADD CONSTRAINT "systems_user_id_users_id_fk" FOREIGN KEY ("user_id") REFERENCES "public"."users"("id") ON DELETE set null ON UPDATE no action;--> statement-breakpoint
CREATE INDEX "system_id_index" ON "jobs" USING btree ("system_id");--> statement-breakpoint
CREATE INDEX "user_name_index" ON "users" USING btree ("user_name");