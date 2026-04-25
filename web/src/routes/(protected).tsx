import { createAsync, redirect } from '@solidjs/router';
import { ErrorBoundary, JSX, Show, Suspense } from 'solid-js';
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
    <ErrorBoundary
      fallback={(error) => {
        return (
          <div class="h-full w-full flex flex-col gap-2 items-center justify-center">
            <p>Something went wrong, try reloading the page.</p>
            <p>{error}</p>
          </div>
        );
      }}
    >
      <Suspense fallback={<Loading />}>
        <Show when={user()} keyed>
          <UserContext.Provider value={user}>
            <Navbar user={user() ?? null} />
            <main class="bg-background text-foreground p-5 h-full w-full">
              {props.children}
            </main>
          </UserContext.Provider>
        </Show>
      </Suspense>
    </ErrorBoundary>
  );
}
