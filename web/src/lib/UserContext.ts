import type { User } from '~/types';
import { createContext, useContext } from 'solid-js';
import { AccessorWithLatest } from '@solidjs/router';

export const UserContext =
  createContext<AccessorWithLatest<User | undefined>>();

export function useUserContext() {
  const value = useContext(UserContext);

  if (!value) {
    throw new Error('Missing context Provider');
  }

  return value;
}
