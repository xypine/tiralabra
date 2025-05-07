import {
  Accessor,
  Component,
  Setter,
  Show,
  createEffect,
  createMemo,
  createSignal,
} from "solid-js";
import { CustomRule } from "./utils";
import { InbuiltRuleSet, WorkerRequest, WorkerResponse } from "./worker";
import Worker from "./worker?worker";
import { Location2D, RuleSet } from "aaltofunktionromautus";

export const Extractor: Component<{
  customRules: Accessor<CustomRule[]>;
  setCustomRules: Setter<CustomRule[]>;
  setRules: Setter<InbuiltRuleSet | CustomRule>;
}> = ({ customRules, setCustomRules, setRules }) => {
  const [extracted, setExtracted] = createSignal<RuleSet<Location2D> | null>(
    null,
  );
  const [waitingForWorker, setWaitingForWorker] = createSignal(false);
  const worker = createMemo(() => {
    console.debug("init worker");
    const w = new Worker();
    w.onmessage = (event) => {
      console.log({ event });
      let data: WorkerResponse = event.data;
      if (data.type === "extracted_rules") {
        console.log("rule", data.result);
        setCustomRules((p) => [
          ...p.filter((r) => r.name !== data.result.name),
          data.result,
        ]);
        setWaitingForWorker(false);
        setExtracted(JSON.parse(data.result.rules));
        setRules(data.result);
        return;
      }
    };
    w.onerror = (event) => {
      console.warn("error", { event });
      throw event.error;
    };
    return w;
  });
  const postMessage = (msg: WorkerRequest) => {
    setWaitingForWorker(true);
    worker().postMessage(msg);
  };
  const [imageDimensions, setImageDimensions] = createSignal<{
    width: number;
    height: number;
  } | null>(null);
  let fileInputRef!: HTMLInputElement;
  const [file, setFile] = createSignal<File | null>(null);
  const [imageData, setImageData] = createSignal<Uint8Array | null>(null);
  const biggerThanRecommended = createMemo(() => {
    const d = imageDimensions();
    if (!d) {
      return false;
    }
    return d.width > 512 || d.height > 512;
  });
  createEffect(async () => {
    const f = file();
    if (f === null) {
      setImageData(null);
      setImageDimensions(null);
    } else {
      // Load image to get dimensions
      const img = new Image();
      img.onload = () => {
        setImageDimensions({ width: img.width, height: img.height });
      };
      img.src = URL.createObjectURL(f);

      // Read file as Uint8Array
      const arrayBuffer = await f.arrayBuffer();
      setImageData(new Uint8Array(arrayBuffer));
    }
  });

  const [n, setN] = createSignal(3);
  const [periodic_input, setPeriodicInput] = createSignal(true);
  const [symmetry, setSymmetry] = createSignal(2);

  const handleFileChange = async (event: Event) => {
    const input = event.target as HTMLInputElement;
    const file = input.files?.[0];
    if (!file) {
      setFile(null);
      return;
    }

    setFile(file);
  };
  createEffect(() => {
    if (file() === null) {
      fileInputRef.value = "";
    }
    setExtracted(null);
  });

  const logBytes = () => {
    const bytes = imageData();
    const name = file()?.name;
    if (bytes && name) {
      console.log("Image bytes:", bytes);
      postMessage({
        type: "extract_rules",
        name: name,
        source: bytes,
        options: {
          n: n(),
          periodic_input: periodic_input(),
          symmetry: symmetry(),
        },
      });
      // setFile(null);
    } else {
      console.log("No image loaded.");
    }
  };

  return (
    <div
      style={{ "font-size": "1rem", padding: "1rem", background: "#1D2021" }}
    >
      <input
        ref={fileInputRef}
        type="file"
        accept="image/png, image/jpeg"
        onChange={handleFileChange}
        disabled={waitingForWorker()}
      />
      <Show when={file()}>
        {(f) => (
          <div>
            <img
              style={{ padding: "1rem", width: "200px", height: "auto" }}
              src={URL.createObjectURL(f())}
            />
          </div>
        )}
      </Show>
      <Show when={imageDimensions()}>
        {(i) => (
          <div
            style={{
              display: "flex",
              gap: "1rem",
              "flex-direction": "column",
              padding: "1rem",
            }}
          >
            <p>
              Dimensions: {i().width} x {i().height}
            </p>
            <label>
              N
              <input value={n()} onChange={(e) => setN(+e.target.value)} />
            </label>
            <label>
              Symmetries
              <input
                value={symmetry()}
                onChange={(e) => setSymmetry(+e.target.value)}
              />
            </label>
            <label>
              Periodic Input
              <input
                type="checkbox"
                checked={periodic_input()}
                onChange={(e) => setPeriodicInput(e.target.checked)}
              />
            </label>
          </div>
        )}
      </Show>
      <Show when={biggerThanRecommended()}>
        <p>
          Using images larger than 512 x 512 IS REALLY NOT RECOMMENDED as the
          resulting ruleset will be most likely immense.
        </p>
      </Show>
      <Show when={imageData()}>
        <button
          onClick={() => {
            alert(`
N
Extracted tiles will contain context of N x N pixels around the tile.
- The larger this is, the more structure will be carried through to the output
 - If N is large enough, all outputs will be identical to the input
 - If N is small enough, outputs may appear random

Symmetry
 1. No symmetry
 2. Vertical symmetry
 3. 
`);
          }}
        >
          Help
        </button>
        <button onClick={logBytes} disabled={waitingForWorker()}>
          Extract Rules
        </button>
      </Show>
      <Show when={extracted()}>
        {(e) => <p>{e().possible.length} tiles extracted</p>}
      </Show>
    </div>
  );
};
