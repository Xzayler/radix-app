import { A, redirect, useSubmission } from '@solidjs/router';
import { action } from '@solidjs/router';
import { createEffect } from 'solid-js';
import { registerWithRedirect } from '~/api/queries';

export default function Signup() {
  const registerAction = action(registerWithRedirect, 'register');

  const registerResponse = useSubmission(registerAction);
  createEffect(() => {
    console.log(registerResponse);
  });

  let pwInput: HTMLInputElement | ((el: HTMLInputElement) => void) | undefined;
  let confPwInput:
    | HTMLInputElement
    | ((el: HTMLInputElement) => void)
    | undefined;

  function passCheck() {
    const confPass = confPwInput as HTMLInputElement;

    if (confPass.value != (pwInput as HTMLInputElement).value) {
      confPass.setCustomValidity("The passwords don't match!");
    } else {
      confPass.setCustomValidity('');
    }
  }

  return (
    <div class="h-full bg-background text-foreground flex flex-col justify-center items-center">
      <p class="text-foreground h-6">{registerResponse.error?.message}</p>
      <form
        class="mt-5 w-full max-w-lg mx-auto flex flex-col"
        method="post"
        action={registerAction}
      >
        <input
          class="p-3.5 bg-background rounded-t border border-ui text-foreground outline-none"
          name="username"
          type="text"
          placeholder="Username"
          required
          pattern="^[A-Za-z0-9_-]+$"
          minLength={3}
          maxLength={16}
        />
        <input
          ref={pwInput}
          class="p-3.5 bg-background rounded-t border border-ui text-foreground outline-none"
          name="password"
          type="password"
          placeholder="Password"
          required
          minLength={6}
          maxLength={32}
        />
        <input
          ref={confPwInput}
          onInput={passCheck}
          class="p-3.5 bg-background rounded-t border border-ui text-foreground outline-none"
          name="confirm-password"
          type="password"
          placeholder="Confirm Password"
          required
          minLength={6}
          maxLength={32}
        />
        <button class="mt-2.5 py-2.5 text-white font-bold rounded bg-accent hover:opacity-90">
          Sign up
        </button>
      </form>
      <div class="mt-5">
        Already have an account?
        <A class="text-accent ml-1" href="/login">
          Log in here
        </A>
      </div>
    </div>
  );
}
