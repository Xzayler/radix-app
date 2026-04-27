import { createSignal, ErrorBoundary } from 'solid-js';
import Matrix from './entryFields/Matrix';
import TextInput from '../forms/TextInput';
import NaturalNumberInput from '../forms/NaturalNumberInput';
import StyledSelect from '../forms/StyledSelect';
import { DigitType } from '~/types';
import NumberInput from '../forms/NumberInput';
import { digitNeedsParam } from '~/lib/validators';
import MatrixInput from '../forms/MatrixInput';
import VectorSetInput from '../forms/VectorSetInput';
import VectorSet from './entryFields/VectorSet';
import { Button } from '@kobalte/core/button';
import { useSubmission } from '@solidjs/router';
import { uploadSystemFromFormWithRedirect } from '~/api/queries';

export default function SystemForm() {
  const [dim, setDim] = createSignal<number>(2);
  const [base, setBase] = createSignal<number[][]>([]);
  const [digitType, setDigitType] = createSignal<DigitType>('Canonical');
  const [param, setParam] = createSignal<number | undefined>(undefined);
  const [digits, setDigits] = createSignal<number[][]>([]);

  const createSystemSubmission = useSubmission(
    uploadSystemFromFormWithRedirect,
  );

  return (
    <form
      class="space-y-3"
      action={uploadSystemFromFormWithRedirect}
      method="post"
    >
      <div class="flex gap-x-3 items-center">
        <div class="flex gap-3 rounded-md bg-highlight p-2 w-min">
          <TextInput
            name="name"
            label={
              <span class="text-sm font-semibold uppercase tracking-wider text-foreground">
                System Name
              </span>
            }
            placeholder="System name"
            minLength={3}
            maxLength={32}
            required={true}
            pattern={'[A-Za-z0-9 _\\-]+'}
          />
          <NaturalNumberInput
            label={
              <span class="text-sm font-semibold uppercase tracking-wider text-foreground">
                Dim
              </span>
            }
            name="dim"
            value={dim()}
            onChange={(n: number | undefined) => {
              setDim(n ?? 1);
            }}
            required={true}
          />
        </div>
        <div class="text-red-500">{createSystemSubmission.error?.message}</div>
      </div>
      <div class="rounded-md bg-highlight flex justify-between p-2 gap-3">
        <div class="basis-1/2">
          <MatrixInput
            label={
              <span class="text-sm font-semibold uppercase tracking-wider text-foreground">
                Base
              </span>
            }
            name="base"
            placeholder="0 1 2..."
            setValue={setBase}
            dim={dim()}
            required={true}
          />
        </div>
        <div class="basis-1/2">
          <span class="text-sm font-semibold uppercase tracking-wider text-foreground">
            Preview
          </span>
          <ErrorBoundary fallback={<Matrix matrix={[]} toShow={dim()} />}>
            <Matrix matrix={base()} toShow={dim()} />
          </ErrorBoundary>
        </div>
      </div>

      <div class="rounded-md bg-highlight p-2 space-y-2">
        <div class="flex gap-3">
          <div class="w-32">
            <StyledSelect<DigitType>
              label={
                <span class="text-sm font-semibold uppercase tracking-wider text-foreground">
                  Digit Type
                </span>
              }
              name="dtype"
              options={[
                'Explicit',
                'Canonical',
                'JCanonical',
                'Symmetric',
                'JSymmetric',
                'Shifted',
                'Adjoint',
              ]}
              value={digitType()}
              onChange={setDigitType}
            />
          </div>
          <NumberInput
            label={
              <span class="text-sm font-semibold uppercase tracking-wider text-foreground">
                Param
              </span>
            }
            name="param"
            value={param()}
            onChange={setParam}
            disabled={!digitNeedsParam(digitType())}
            required={digitNeedsParam(digitType())}
          />
        </div>
        <div>
          <VectorSetInput
            label={
              <span class="text-sm font-semibold uppercase tracking-wider text-foreground">
                Explicit Digits
              </span>
            }
            name="digits"
            dim={dim()}
            placeholder="0 1 2..."
            setValue={setDigits}
            disabled={digitType() != 'Explicit'}
          />
        </div>
        <div>
          <ErrorBoundary fallback={<VectorSet showAll={true} vectors={[]} />}>
            <VectorSet showAll={true} vectors={digits()} />
          </ErrorBoundary>
        </div>
      </div>
      <Button type="submit" class="px-3 py-2 rounded-md bg-accent">
        <span>Create</span>
      </Button>
    </form>
  );
}
