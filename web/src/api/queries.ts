import { query, redirect } from '@solidjs/router';
import { getCurrentUser, register } from '~/api/server';

export const getCurrentUserQueryWithRedirect = query(async () => {
  'use server';
  const user = await getCurrentUser();
  if (!user) {
    throw redirect('/login');
  }
  return user;
}, 'getCurrentUserRedirect');

export const isLoggedInQuery = query(async () => {
  'use server';
  const user = await getCurrentUser();
  if (user) {
    throw redirect('/home');
  }
}, 'isLoggedIn');

export const registerWithRedirect = async (formData: FormData) => {
  'use server';
  await register(formData);
  throw redirect('/home', {
    revalidate: [getCurrentUserQueryWithRedirect.key, isLoggedInQuery.key],
  });
};
