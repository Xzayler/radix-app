import type { System } from '~/types';
import { createContext, useContext } from 'solid-js';
import { AccessorWithLatest } from '@solidjs/router';

export const SystemContext =
  createContext<AccessorWithLatest<System | null | undefined>>();

export function useSystemContext() {
  const value = useContext(SystemContext);

  if (!value) {
    throw new Error('Missing context Provider');
  }

  return value;
}
