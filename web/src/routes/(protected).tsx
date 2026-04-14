import { createAsync, redirect } from '@solidjs/router';
import { JSX, Show, Suspense } from 'solid-js';
import { getCurrentUserQueryWithRedirect } from '~/api/queries';
import Loading from '~/components/shared/Loading';
import Navbar from '~/components/shared/Navbar';
import { UserContext } from '~/lib/UserContext';

export const route = {
  preload() {
    return getCurrentUserQueryWithRedirect();
  },
};

export default function ProtectedLayout(props: { children: JSX.Element }) {
  const user = createAsync(() => getCurrentUserQueryWithRedirect(), {
    deferStream: true,
  });

  return (
    <Suspense fallback={<Loading />}>
      <Show when={user()} keyed>
        <UserContext.Provider value={user}>
          <Navbar />
          <main class="bg-background text-foreground p-5 min-h-dvh h-full w-full">
            {props.children}
          </main>
        </UserContext.Provider>
      </Show>
    </Suspense>
  );
}
