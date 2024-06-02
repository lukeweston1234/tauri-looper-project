import { For } from "solid-js";

export default function AudioVisualization(props: {
  downsampledData: number[];
}) {
  // Normalize the data for better visualization
  // const normalizedData = () =>
  //   props.downsampledData.map((value) =>
  //     Math.round(
  //       (value / Math.max(...props.downsampledData.map(Math.abs))) * 100,
  //     ),
  //   );

  const maxAmplitude = () => Math.max(...props.downsampledData.map(Math.abs));

  const normalizedData = () =>
    props.downsampledData.map((x) => (Math.abs(x) / maxAmplitude()) * 100);

  return (
    <div class="flex h-32 w-full items-center justify-between p-3">
      <For each={normalizedData()}>
        {(chunk) => (
          <div class="w-[1px] bg-appPrimary" style={{ height: `${chunk}%` }} />
        )}
      </For>
    </div>
  );
}
