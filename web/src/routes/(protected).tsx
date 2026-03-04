import { createAsync, redirect } from '@solidjs/router';
import { JSX, Show, Suspense } from 'solid-js';
import { getCurrentUserQuery } from '~/api/queries';
import Loading from '~/components/shared/Loading';
import Navbar from '~/components/shared/Navbar';
import { UserContext } from '~/lib/UserContext';
import { User } from '~/types';

async function getCurrentUserWithRedirect() {
  const user = await getCurrentUserQuery();
  if (!user) throw redirect('/login');
  return user;
}

export const route = {
  preload() {
    return getCurrentUserWithRedirect();
  },
};

export default function ProtectedLayout(props: { children: JSX.Element }) {
  const user = createAsync(() => getCurrentUserWithRedirect());

  return (
    <Suspense fallback={<Loading />}>
      <Show when={user()} keyed>
        <UserContext.Provider value={user}>
          <Navbar />
          {props.children}
        </UserContext.Provider>
      </Show>
    </Suspense>
  );
}
