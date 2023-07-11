export interface GasInfo {
    id: number | undefined | null;
    pricePerGallon: number;
    totalCost: number;
    gallons: number;
    aTripometer: number;
    bTripometer: number;
    totalTripometer: number;
    timeRecorded: Date;
    city: string;
    state: string;
}

export interface StringGasInfo {
    id: number | undefined | null;
    pricePerGallon: string;
    totalCost: string;
    gallons: string;
    aTripometer: string;
    bTripometer: string;
    totalTripometer: number;
    timeRecorded: string;
    city: string;
    state: string;
}

export interface GasStats {
    totalCost: number;
    totalGallons: number;
    avgPpg: number;
    avgMpg: number;
    avgATrip: number;
    avgFillSize: number;
}

export interface StringGasStats {
    totalCost: string;
    totalGallons: string;
    avgPpg: string;
    avgMpg: string;
    avgATrip: string;
    avgFillSize: string;
}

export class GasInfoConversions {
    public static gasInfoStringsToNums(unParsed: StringGasInfo): GasInfo {
        return {
            id: unParsed.id,
            pricePerGallon: parseFloat(unParsed.pricePerGallon),
            totalCost: parseFloat(unParsed.totalCost),
            gallons: parseFloat(unParsed.gallons),
            aTripometer: parseFloat(unParsed.aTripometer),
            bTripometer: parseFloat(unParsed.bTripometer),
            totalTripometer: parseInt(unParsed.totalTripometer),
            timeRecorded: new Date(unParsed.timeRecorded),
            city: unParsed.city,
            state: unParsed.state,
        };
    }

    public static statsStringsToNums(unParsed: StringGasStats): GasStats {
        return {
            totalCost: parseFloat(unParsed.totalCost),
            totalGallons: parseFloat(unParsed.totalGallons),
            avgPpg: parseFloat(unParsed.avgPpg),
            avgMpg: parseFloat(unParsed.avgMpg),
            avgATrip: parseFloat(unParsed.avgATrip),
            avgFillSize: parseFloat(unParsed.avgFillSize),
        };
    }

    public static gasInfoToJson(unParsed: GasInfo): string {
        let parsed: StringGasInfo = {
            id: unParsed.id,
            pricePerGallon: unParsed.pricePerGallon.toString(),
            totalCost: unParsed.totalCost.toString(),
            gallons: unParsed.gallons.toString(),
            aTripometer: unParsed.aTripometer.toString(),
            bTripometer: unParsed.bTripometer.toString(),
            totalTripometer: unParsed.totalTripometer,
            timeRecorded: unParsed.timeRecorded.toISOString(),
            city: unParsed.city,
            state: unParsed.state
        }

        return JSON.stringify(parsed)
    }
}
