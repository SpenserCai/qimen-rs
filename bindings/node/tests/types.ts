import { calculate, calculateJson, type Chart, type ChartRequest, type PalaceNumber } from '../index';

const request: ChartRequest = { year: 2024, month: 2, day: 10, hour: 12 };
const chart: Chart = calculate(request);
const palace: PalaceNumber = chart.palaces[0].number;
const serialized: string = calculateJson(JSON.stringify(request));
void palace;
void serialized;

// @ts-expect-error unknown conventions must not type-check
calculate({ ...request, day_boundary: 'solar_midnight' });
// @ts-expect-error Gregorian dates require integer number fields
calculate({ ...request, year: '2024' });
// @ts-expect-error unknown request fields must not type-check
calculate({ ...request, typo: 1 });
