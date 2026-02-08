import 'dotenv/config';
import { drizzle } from "drizzle-orm/node-postgres"
import { schema } from "./schema";

const url = process.env.DATABASE_URL;
if (!url) throw new Error('Database url is missing');

console.log("Connecting to " + url)
export const db = drizzle(url, { schema });