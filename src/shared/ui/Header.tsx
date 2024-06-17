import { Accessor, Show } from "solid-js";

export default function Header(props: {
  isRecording: Accessor<boolean>;
  isPlaying: Accessor<boolean>;
  isMetronomeOn: Accessor<boolean>;
  onRecord: () => void;
  onPlay: () => void;
  onStop: () => void;
  onMetronomeToggle: () => void;
}) {
  return (
    <div class="relative flex h-[74px] w-full items-center justify-center gap-6 bg-black">
      <div class="absolute left-0 flex items-center gap-6 pl-6">
        <svg
          width="60"
          height="40"
          viewBox="0 0 60 40"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
          class="left-0 fill-appPrimary"
        >
          <g clip-path="url(#clip0_1_894)">
            <path d="M56.25 1.25C56.25 1.94036 56.8096 2.5 57.5 2.5H58.75C59.4404 2.5 60 1.94036 60 1.25C60 0.559645 59.4404 0 58.75 0H57.5C56.8096 0 56.25 0.559645 56.25 1.25Z" />
            <path d="M20 40H26.0723L24.3045 38.2322C23.8357 37.7634 23.1998 37.5 22.5368 37.5H20C10.335 37.5 2.5 29.665 2.5 20C2.5 10.335 10.335 2.50001 20 2.5H40C49.665 2.5 57.5 10.335 57.5 20C57.5 29.665 49.665 37.5 40 37.5H32.5184C31.5238 37.5 30.57 37.1049 29.8667 36.4017L27.7957 34.3306C26.6236 33.1585 25.0338 32.5 23.3762 32.5H20C13.0964 32.5 7.5 26.9036 7.5 20C7.5 13.0964 13.0964 7.5 20 7.5H40C46.9036 7.5 52.5 13.0964 52.5 20C52.5 26.9036 46.9036 32.5 40 32.5H35.1961C34.2015 32.5 33.2477 32.1049 32.5444 31.4017L30.4733 29.3306C29.3012 28.1585 27.7115 27.5 26.0539 27.5H20C15.8579 27.5 12.5 24.1421 12.5 20C12.5 15.8579 15.8579 12.5 20 12.5H40C44.1421 12.5 47.5 15.8579 47.5 20C47.5 24.0916 44.2235 27.418 40.1512 27.4985L40.1504 27.5H38.3211C37.3265 27.5 36.3727 27.1049 35.6694 26.4017L33.5983 24.3306C32.6366 23.3688 31.3937 22.7529 30.0628 22.5628L30 22.5H20C18.6193 22.5 17.5 21.3807 17.5 20C17.5 18.6193 18.6193 17.5 20 17.5H40C41.3807 17.5 42.5 18.6193 42.5 20C42.5 21.3807 41.3807 22.5 40 22.5H35L36.7678 24.2678C37.2366 24.7366 37.8725 25 38.5355 25H40C42.7614 25 45 22.7614 45 20C45 17.2386 42.7614 15 40 15H20C17.2386 15 15 17.2386 15 20C15 22.7614 17.2386 25 20 25H29.1789C30.1735 25 31.1273 25.3951 31.8306 26.0983L33.9017 28.1694C35.0738 29.3415 36.6635 30 38.3211 30H40.625V29.9808C45.8567 29.6582 50 25.3129 50 20C50 14.4772 45.5228 10 40 10H20C14.4772 10 10 14.4772 10 20C10 25.5229 14.4772 30 20 30H26.0539C27.0485 30 28.0023 30.3951 28.7056 31.0983L30.7767 33.1694C31.9488 34.3415 33.5385 35 35.1961 35H40C48.2843 35 55 28.2843 55 20C55 11.7157 48.2843 5 40 5H20C11.7157 5 5 11.7157 5 20C5 28.2843 11.7157 35 20 35H23.3762C24.3708 35 25.3246 35.3951 26.0279 36.0983L28.099 38.1694C29.2711 39.3415 30.8608 40 32.5184 40H40C51.0457 40 60 31.0457 60 20C60 8.9543 51.0457 -9.65645e-07 40 0L20 4.13264e-06C8.9543 5.09829e-06 -9.65645e-07 8.95431 0 20C9.65645e-07 31.0457 8.95431 40 20 40Z" />
          </g>
          <defs>
            <clipPath id="clip0_1_894">
              <rect width="60" height="40" fill="white" />
            </clipPath>
          </defs>
        </svg>
        <div class="mr-auto flex gap-6">
          <span class="text-[10px]">MEM 32%</span>
          <span class="text-[10px]">CPU 24%</span>
          <span class="text-[10px]">TEMP 54°C</span>
        </div>
      </div>

      <div class="flex gap-3">
        <Show
          when={props.isPlaying()}
          fallback={
            <button onClick={props.onPlay}>
              <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                stroke-width="2"
                stroke="currentColor"
                class="size-6"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  d="M5.25 5.653c0-.856.917-1.398 1.667-.986l11.54 6.347a1.125 1.125 0 0 1 0 1.972l-11.54 6.347a1.125 1.125 0 0 1-1.667-.986V5.653Z"
                />
              </svg>
            </button>
          }
        >
          <button onClick={props.onStop}>
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 24 24"
              fill="currentColor"
              class="size-6"
            >
              <path
                fill-rule="evenodd"
                d="M6.75 5.25a.75.75 0 0 1 .75-.75H9a.75.75 0 0 1 .75.75v13.5a.75.75 0 0 1-.75.75H7.5a.75.75 0 0 1-.75-.75V5.25Zm7.5 0A.75.75 0 0 1 15 4.5h1.5a.75.75 0 0 1 .75.75v13.5a.75.75 0 0 1-.75.75H15a.75.75 0 0 1-.75-.75V5.25Z"
                clip-rule="evenodd"
              />
            </svg>
          </button>
        </Show>

        <button onClick={() => props.onRecord()}>
          <div
            class={`size-4 rounded-full ${props.isRecording() ? "border-none bg-appPrimary" : "border-2 border-appPrimary"} `}
          />
        </button>
      </div>
      <div class="absolute right-0 flex flex-nowrap items-center gap-1 pr-6">
        <button onClick={() => props.onMetronomeToggle()}>
          <svg
            width="16"
            height="16"
            viewBox="0 0 20 22"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
            class={!props.isMetronomeOn() ? "opacity-60" : ""}
          >
            <path
              d="M9.55556 14.6364L18 1M6.33333 4.50333H13L18 21H2L6.33333 4.50333Z"
              stroke="#C8BCEA"
              stroke-width="2"
            />
          </svg>
        </button>
        <span class="text-sm">120</span>
      </div>
    </div>
  );
}
