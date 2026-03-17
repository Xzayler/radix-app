import katex from 'katex';

export default function BaseMatrix(props: { matrix: number[][] }) {
  let latexString;
  if (props.matrix.length > 4) {
    const lastRow = new Array(5).fill('...').join(' & ');
    latexString = props.matrix
      .slice(0, 4)
      .map((row) => row.slice(0, 4).join(' & ') + ' & ...')
      .concat(lastRow)
      .join(' \\\\ ');
  } else {
    latexString = props.matrix.map((row) => row.join(' & ')).join(' \\\\ ');
  }

  const html = katex.renderToString(
    `\\begin{bmatrix} ${latexString} \\end{bmatrix}`,
    {
      throwOnError: false,
    },
  );

  return <div class="text-foreground" innerHTML={html}></div>;
}
