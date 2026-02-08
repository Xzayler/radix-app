import * as argon2 from "argon2";
import { redirect } from "@solidjs/router";
import { useSession } from "vinxi/http";
import { getByUserName, getUserById, insertUser } from "~/db/operations";
import { User } from "~/types";
import { UserDbInsert } from "~/db/dbTypes";

async function login(userName: string, password: string) : Promise<User> {
  const user = await getByUserName(userName);
  if (!user || !(await argon2.verify(user.password, password))) throw new Error("Invalid login");
  return user as User;
}

async function register(userName: string, password: string) {
  const existingUser = await getByUserName(userName);
  if (existingUser) throw new Error("User already exists");
  const newUser: UserDbInsert = {
    userName,
    password: await argon2.hash(password),
  }
  return await insertUser(newUser);
}

export async function loginOrRegister(formData: FormData) {
  const username = String(formData.get("username"));
  const password = String(formData.get("password"));
  const loginType = String(formData.get("loginType"));
  // let error = validateUsername(username) || validatePassword(password);
  // if (error) return new Error(error);

  try {
    const user: User = await (loginType !== "login"
      ? register(username, password)
      : login(username, password));
    const session = await getSession();
    await session.update(d => {
      d.userId = user.id;
    });
  } catch (err) {
    return err as Error;
  }
  throw redirect("/");
}

function getSession() {
  return useSession({
    password: process.env.SESSION_SECRET ?? "areallylongsecretthatyoushouldreplace"
  });
}

export async function logout() {
  const session = await getSession();
  await session.update(d => (d.userId = undefined));
  throw redirect("/login");
}

export async function getUser() {
  const session = await getSession();
  const userId = session.data.userId;
  if (userId === undefined) throw redirect("/login");

  try {
    // const user = client.select().from(Users).where(eq(Users.id, userId)).get();
    const user = await getUserById(userId);
    if (!user) throw redirect("/login");
    return { id: user.id, username: user.userName };
  } catch {
    throw logout();
  }
}
