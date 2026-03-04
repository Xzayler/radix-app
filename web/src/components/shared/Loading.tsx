import { createSignal } from 'solid-js';

export default function Loading() {
  const dots = ['', '.', '..', '...'];

  const [step, setStep] = createSignal<0 | 1 | 2 | 3>(0);

  function increaseStep() {
    setTimeout(() => {
      setStep(((step() + 1) % 4) as 0 | 1 | 2 | 3);
      increaseStep();
    }, 400);
  }
  increaseStep();

  return (
    <div class="w-full h-full flex gap-2 flex-col justify-center items-center">
      <div class="max-w-20 h-auto">{/* Some kind of Icon? */}</div>
      <div>{'Loading' + dots[step()]}</div>
    </div>
  );
}
