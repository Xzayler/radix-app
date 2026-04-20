'use server';
import { verify, hash } from 'argon2';
import { useSession } from 'vinxi/http';
import {
  getUserByUserName,
  getUserById,
  insertUser,
} from '~/lib/db/operations';
import { User } from '~/types';
import { UserDbInsert } from '~/lib/db/dbTypes';
import { validateInputAsString, validateUsername } from './validators';

type SessionData = {
  userId: number | undefined;
};

function getUserSession() {
  const password: string = process.env.USER_SESSION_PASSWORD!;
  return useSession<SessionData>({
    password: password,
  });
}

async function setCurrentUser(user: User) {
  const session = await getUserSession();
  await session.update((data) => ((data.userId = user.id), data));
}

function processForm(formData: FormData): {
  userName: string;
  password: string;
} {
  const userName = validateUsername(formData.get('username'), 'Username');
  const password = validateInputAsString(formData.get('password'), 'Password');

  return { userName, password };
}

export async function login(formData: FormData) {
  const { userName, password } = processForm(formData);

  let user;
  try {
    user = await getUserByUserName(userName);
  } catch (e) {
    console.log(e);
    throw new Error("Database Error: couldn't get user");
  }

  if (!user || !(await verify(user.password, password)))
    throw new Error('Invalid login');

  setCurrentUser(user);
}

export async function register(formData: FormData) {
  const { userName, password } = processForm(formData);
  const existingUser = await getUserByUserName(userName);
  if (existingUser) throw new Error('User already exists');
  const newUser: UserDbInsert = {
    userName,
    password: await hash(password),
  };
  const createdUser = await insertUser(newUser);
  setCurrentUser(createdUser);
}

export async function logout() {
  const session = await getUserSession();
  await session.update((d) => (d.userId = undefined));
}

export async function getCurrentUser(): Promise<User | null> {
  const session = await getUserSession();
  const userId = session.data.userId;
  if (!userId) return null;

  try {
    const user = await getUserById(userId);
    if (!user) return null;
    return user as User;
  } catch (e) {
    const session = await getUserSession();
    await session.update((d) => (d.userId = undefined));
    return null;
  }
}

export async function getLoggedInUser(): Promise<User> {
  const user = await getCurrentUser();
  if (!user) {
    throw new Error('No user logged in');
  }
  return user;
}

export const guestLogin = async () => {
  const guestUser: User = { id: 1, userName: 'Guest' };
  setCurrentUser(guestUser);
};
