import { Button } from '@kobalte/core/button';
import { A, action } from '@solidjs/router';
import { Show } from 'solid-js';
import { logoutWithRedirect } from '~/api/queries';
import { logout } from '~/api/server';
import { User } from '~/types';

export default function Navbar(props: { user: User | null | undefined }) {
  return (
    <header class="sticky text-foreground top-0 rounded-b-md bg-highlight overflow-hidden">
      <div class="mx-auto max-w-6xl flex items-center py-2 justify-between">
        <div class="basis-1/4"></div>
        <div class="grow flex items-center justify-center gap-6">
          <A
            class="hover:underline underline-offset-2 decoration-2 text-md font-semibold uppercase tracking-wider text-foreground"
            href="/"
          >
            Home
          </A>
          <A
            class="hover:underline underline-offset-2 decoration-2 text-md font-semibold uppercase tracking-wider text-foreground"
            href="/systems"
          >
            Systems
          </A>
          <A
            class="hover:underline underline-offset-2 decoration-2 text-md font-semibold uppercase tracking-wider text-foreground"
            href="/systems/new"
          >
            Add System
          </A>
        </div>
        <Show
          when={props.user}
          fallback={
            <div class="basis-1/4 flex gap-x-2 items-center justify-end">
              <Button class="bg-accent font-bold hover:opacity-90 rounded-md px-2 py-1 cursor-pointer">
                <A href="/login">Log In</A>
              </Button>
              <Button class="px-2 py-1 cursor-pointer">
                <A class="hover:underline underline-offset-2" href="/signup">
                  Sign Up
                </A>
              </Button>
            </div>
          }
        >
          <div class="flex items-center gap-x-2 basis-1/4 justify-end">
            <span class="text-md font-semibold uppercase tracking-wider text-foreground">
              {props.user!.userName}
            </span>
            <form action={logoutWithRedirect} method="post">
              <Button
                class="bg-accent rounded-md px-2 py-1 cursor-pointer"
                type="submit"
              >
                Log Out
              </Button>
            </form>
          </div>
        </Show>
      </div>
    </header>
  );
}
