CREATE TYPE "public"."input_type" AS ENUM('Explicit', 'Canonical', 'J-Canonical', 'Dense', 'Adjoined', 'Symmetric', 'Shifted');--> statement-breakpoint
CREATE TYPE "public"."job_type" AS ENUM('Walk', 'Decision', 'Classification');--> statement-breakpoint
CREATE TYPE "public"."status" AS ENUM('Pending', 'Running', 'Success', 'Failed');--> statement-breakpoint
CREATE TABLE "jobs" (
	"id" integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY (sequence name "jobs_id_seq" INCREMENT BY 1 MINVALUE 1 MAXVALUE 2147483647 START WITH 1 CACHE 1),
	"user_id" integer NOT NULL,
	"system_id" integer NOT NULL,
	"status" "status" NOT NULL,
	"job_type" "job_type" NOT NULL,
	"output_uri" text,
	"created_at" timestamp (0) with time zone DEFAULT now(),
	"started_at" timestamp (0) with time zone,
	"finished_at" timestamp (0) with time zone
);
--> statement-breakpoint
CREATE TABLE "systems" (
	"id" integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY (sequence name "systems_id_seq" INCREMENT BY 1 MINVALUE 1 MAXVALUE 2147483647 START WITH 1 CACHE 1),
	"dimension" integer,
	"base" integer[] NOT NULL,
	"digit_type" "input_type" NOT NULL,
	"is_gns" boolean,
	"signature" integer[],
	"input_uri" text NOT NULL
);
--> statement-breakpoint
CREATE TABLE "users" (
	"id" integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY (sequence name "users_id_seq" INCREMENT BY 1 MINVALUE 1 MAXVALUE 2147483647 START WITH 1 CACHE 1),
	"user_name" text NOT NULL,
	"password" text NOT NULL,
	CONSTRAINT "users_user_name_unique" UNIQUE("user_name")
);
--> statement-breakpoint
ALTER TABLE "jobs" ADD CONSTRAINT "jobs_user_id_users_id_fk" FOREIGN KEY ("user_id") REFERENCES "public"."users"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
ALTER TABLE "jobs" ADD CONSTRAINT "jobs_system_id_systems_id_fk" FOREIGN KEY ("system_id") REFERENCES "public"."systems"("id") ON DELETE no action ON UPDATE no action;--> statement-breakpoint
CREATE INDEX "system_id_index" ON "jobs" USING btree ("system_id");--> statement-breakpoint
CREATE INDEX "user_id_index" ON "jobs" USING btree ("user_id");--> statement-breakpoint
CREATE INDEX "user_name_index" ON "users" USING btree ("user_name");