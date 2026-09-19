import { calculate, calculateJson, type Chart, type ChartRequest, type ExtensionOptions, type PalaceNumber } from '../index';

const request: ChartRequest = { year: 2024, month: 2, day: 10, hour: 12 };
const chart: Chart = calculate(request);
const palace: PalaceNumber = chart.palaces[0].number;
const serialized: string = calculateJson(JSON.stringify(request));
void palace;
void serialized;

const extensions: ExtensionOptions = { day_horse: 'day_branch_three_harmony' };
const extended: Chart = calculate({ ...request, extensions });
void extended;

// @ts-expect-error unknown conventions must not type-check
calculate({ ...request, day_boundary: 'solar_midnight' });
// @ts-expect-error Gregorian dates require integer number fields
calculate({ ...request, year: '2024' });
// @ts-expect-error unknown request fields must not type-check
calculate({ ...request, typo: 1 });
// @ts-expect-error extension conventions must be known rule names
calculate({ ...request, extensions: { day_horse: 'automatic' } });
// @ts-expect-error unknown extension names must not type-check
calculate({ ...request, extensions: { luck: true } });
// @ts-expect-error annotations require an explicit convention, not a boolean
calculate({ ...request, extensions: { hidden_stems: true } });
