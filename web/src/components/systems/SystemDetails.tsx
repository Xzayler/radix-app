import { Digits, ExplicitDigits, System } from '~/types';
import StarIcon from '../shared/StarIcon';
import { createEffect, createSignal, JSX, Show } from 'solid-js';
import VectorSet from './entryFields/VectorSet';
import Matrix from './entryFields/Matrix';
import { favourite, unFavourite } from '~/api/server';
import { Button } from '@kobalte/core/button';
import AddIcon from '../shared/AddIcon';
import { A } from '@solidjs/router';

function Field(props: { label: string; children: JSX.Element }) {
  return (
    <div class="flex flex-col gap-1.5">
      <span class="text-sm font-semibold uppercase tracking-wider text-muted-foreground">
        {props.label}
      </span>
      <div class="text-sm text-foreground">{props.children}</div>
    </div>
  );
}

function getDigitParam(digits: Digits) {
  switch (digits.type) {
    case 'Shifted':
      return digits.shift;
    case 'JCanonical':
    case 'JSymmetric':
      return digits.jValue;
    default:
      return null;
  }
}

export default function SystemDetails(props: { system: System }) {
  const [isFavourited, setIsFavourited] = createSignal(
    props.system.isFavourited,
  );

  createEffect(() => {
    setIsFavourited(props.system.isFavourited);
  });

  const toggleFavourite = async () => {
    const nextValue = !isFavourited();
    setIsFavourited(nextValue);

    try {
      if (nextValue) {
        await favourite(props.system.id);
      } else {
        await unFavourite(props.system.id);
      }
    } catch (error) {
      setIsFavourited(!nextValue);
      console.error('Failed to update favourite status', error);
    }
  };

  const digitsCount =
    props.system.digits.type == 'Explicit'
      ? (props.system.digits as ExplicitDigits).values.length
      : 0;

  const digitsParam = getDigitParam(props.system.digits);

  return (
    <div class="mx-auto max-w-6xl space-y-8">
      <div class="flex items-center gap-4">
        <div class="space-y-1">
          <p class="text-sm uppercase tracking-wider text-faint">
            System id: {props.system.id}
          </p>
          <h1 class="text-2xl font-semibold leading-tight">
            {props.system.name}
          </h1>
        </div>
        <Button
          class={
            'h-10 aspect-square cursor-pointer hover:text-yellow-500 ' +
            (isFavourited() ? 'text-yellow-500' : 'text-faint')
          }
          onClick={(e) => {
            e.preventDefault();
            toggleFavourite();
          }}
        >
          <StarIcon toFill={isFavourited()} />
        </Button>
        <Button class="ml-auto text-lg rounded-md bg-accent cursor-pointer px-1 pr-2 hover:scale-105 transition-transform">
          <A
            href={`/systems/${props.system.id}/jobs`}
            class="flex items-center"
          >
            <div class="flex items-center">
              <span>{'System Analyses >'}</span>
            </div>
          </A>
        </Button>
      </div>

      <section class="rounded-lg bg-highlight">
        <div class="grid grid-cols-1 gap-x-8 gap-y-6 p-6 md:grid-cols-3">
          <Field label="Dim">{props.system.dimension}</Field>
          <Field label="Digit Type">
            {props.system.digits.type + (digitsParam ? `(${digitsParam})` : '')}
          </Field>
          <Field label="GNS">
            {props.system.isGns === null
              ? '-'
              : props.system.isGns
                ? 'Yes'
                : 'No'}
          </Field>
        </div>

        <div class="p-6">
          <span class="text-sm font-semibold uppercase tracking-wider">
            Base
          </span>
          <div class="mt-3 flex items-center justify-center overflow-x-auto rounded-md bg-highlightextra p-4 text-center font-mono text-sm text-muted-foreground">
            <Matrix
              matrix={props.system.base}
              toShow={props.system.dimension}
            />
          </div>
        </div>

        <Show when={props.system.digits.type == 'Explicit'}>
          <div class="p-6">
            <div class="flex items-baseline gap-4">
              <span class="text-sm font-semibold uppercase tracking-wider">
                Digits
              </span>
              <span class="text-sm">{digitsCount}</span>
            </div>
            <div class="mt-3 overflow-x-auto rounded-md bg-highlightextra p-4">
              {digitsCount === 0 ? (
                <div class="py-2 text-center text-sm">-</div>
              ) : (
                <div class="flex items-center gap-3 whitespace-nowrap font-mono text-sm">
                  <VectorSet
                    vectors={(props.system.digits as ExplicitDigits).values}
                    toShow={digitsCount}
                  />
                </div>
              )}
            </div>
          </div>
        </Show>

        <div class="p-6">
          <span class="text-sm font-semibold uppercase tracking-wider text-muted-foreground">
            Signature
          </span>
          <div class="mt-3 overflow-x-auto rounded-md bg-highlightextra p-4">
            {!props.system.signature?.length ? (
              <div class="py-2 text-center text-sm text-muted-foreground">
                -
              </div>
            ) : (
              <div class="whitespace-nowrap font-mono text-sm text-foreground">
                ({props.system.signature.join(', ')})
              </div>
            )}
          </div>
        </div>

        <div class="px-6 py-4">
          <Field label="Last Job">
            {props.system.lastJob ? props.system.lastJob.toLocaleString() : '-'}
          </Field>
        </div>
      </section>
    </div>
  );
}
