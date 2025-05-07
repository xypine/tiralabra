import { OverlappingBitmapExtractorOptions } from "aaltofunktionromautus";

export type Seed = {
  value: number;
  allowRandomization: boolean;
};

export function pickRandomSeed() {
  return Math.floor(Math.random() * 1000000000000);
}

export type CustomRule = {
  name: string;
  options: OverlappingBitmapExtractorOptions;
  rules: string;
};
