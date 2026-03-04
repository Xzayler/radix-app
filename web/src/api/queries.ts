import { query } from '@solidjs/router';
import { getCurrentUser } from '~/api/server';

export const getCurrentUserQuery = query(getCurrentUser, 'getCurrentUser');
