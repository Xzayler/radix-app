'use server';
import {
  login as li,
  getCurrentUser as gcu,
  logout as lo,
  register as rg,
  guestLogin as gl,
} from '~/lib/auth';

// Auth
// TODO: Move formData handling here.
export const login = li;
export const getCurrentUser = gcu;
export const logout = lo;
export const register = rg;
export const guestLogin = gl;
