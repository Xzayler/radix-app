import { action, query, redirect } from '@solidjs/router';
import {
  getCurrentUser,
  logout,
  register,
  uploadSystemFromFile,
  uploadSystemFromForm,
} from '~/api/server';

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
    throw redirect('/');
  }
}, 'isLoggedIn');

export const registerWithRedirect = async (formData: FormData) => {
  'use server';
  await register(formData);
  throw redirect('/', {
    revalidate: [getCurrentUserQueryWithRedirect.key, isLoggedInQuery.key],
  });
};

export const logoutWithRedirect = action(async () => {
  'use server';
  await logout();
  throw redirect('/', {
    revalidate: [getCurrentUserQueryWithRedirect.key, isLoggedInQuery.key],
  });
}, 'logout');

export const uploadSystemFromFormWithRedirect = action(
  async (formData: FormData) => {
    'use server';
    const system = await uploadSystemFromForm(formData);
    throw redirect(`/systems/${system.id}`);
  },
  'uploadSystem',
);

export const uploadSystemFromFileWithRedirect = action(
  async (formData: FormData) => {
    'use server';
    const system = await uploadSystemFromFile(formData);
    throw redirect(`/systems/${system.id}`);
  },
  'uploadSystem',
);
