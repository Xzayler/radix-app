import { A, useSubmission } from '@solidjs/router';
import { action } from '@solidjs/router';
import { registerWithRedirect } from '~/api/queries';
import TextInput from '../forms/TextInput';
import { TextField } from '@kobalte/core/text-field';

export default function Signup() {
  const registerAction = action(registerWithRedirect, 'register');

  const registerResponse = useSubmission(registerAction);

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
    <div class="h-full bg-background text-foreground flex flex-col justify-center items-center pt-4">
      <p class="text-red-500 h-6 font-semibold">
        {registerResponse.error?.message}
      </p>
      <form
        class="mt-1 w-full max-w-lg mx-auto flex flex-col"
        method="post"
        action={registerAction}
      >
        <input
          class="p-3.5 bg-background rounded-t border border-ui text-foreground outline-none"
          name="username"
          type="text"
          placeholder="Username"
          required
          pattern={'[A-Za-z0-9 _\\-]+'}
          minLength={3}
          maxLength={32}
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
        <button class="mt-2.5 py-2.5 text-white font-bold rounded bg-accent hover:opacity-90 cursor-pointer">
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
