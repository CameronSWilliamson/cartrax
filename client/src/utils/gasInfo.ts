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
    totalTripometer: string;
    timeRecorded: string;
    city: string;
    state: string;
}

export class ConvertGasInfo {
    public static stringsToNums(unParsed: StringGasInfo): GasInfo {
        return {
            id: unParsed.id,
            pricePerGallon: parseFloat(unParsed.pricePerGallon),
            totalCost: parseFloat(unParsed.totalCost),
            gallons: parseFloat(unParsed.gallons),
            aTripometer: parseFloat(unParsed.aTripometer),
            bTripometer: parseFloat(unParsed.bTripometer),
            totalTripometer: parseFloat(unParsed.totalTripometer),
            timeRecorded: new Date(unParsed.timeRecorded),
            city: unParsed.city,
            state: unParsed.state,
        };
    }
}
