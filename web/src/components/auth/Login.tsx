import { A, action, createAsync, useSubmission } from '@solidjs/router';
import { login, guestLogin } from '~/api/server';
import { Switch, Match } from 'solid-js';
import { Navigate } from '@solidjs/router';
import Loading from '../shared/Loading';
import { isLoggedInQuery } from '~/api/queries';

export const route = {
  preload: async () => {
    isLoggedInQuery();
  },
};

export default function Login() {
  const user = createAsync(() => isLoggedInQuery(), { deferStream: true });
  user();

  const loginAction = action(login, 'login');
  const loginSubmission = useSubmission(loginAction);

  return (
    <div class="h-full flex flex-col justify-center items-center">
      <div class="text-foreground h-6">
        {loginSubmission.pending ? '' : (loginSubmission.error?.message ?? '')}
      </div>
      <form
        action={loginAction}
        method="post"
        class="mt-5 w-full max-w-lg mx-auto flex flex-col"
      >
        <input
          class="p-3.5 bg-background rounded-t border border-ui text-foreground outline-none"
          name="username"
          type="text"
          placeholder="Username"
          // required
          // minLength={3}
          // maxLength={32}
        />
        <input
          class="p-3.5 bg-background rounded-b border border-ui text-foreground outline-none"
          name="password"
          type="password"
          placeholder="Password"
          // required
        />
        <button
          type="submit"
          class="mt-2.5 py-2.5 text-white font-bold rounded bg-accent hover:opacity-90"
        >
          Log in
        </button>
      </form>
      <div class="mt-5 text-foreground">
        Don't have an account?
        <A class="text-accent ml-1 inline-block" href="/signup">
          Sign up here
        </A>
      </div>
    </div>
  );
}
