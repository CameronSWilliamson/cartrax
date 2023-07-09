export default interface GasInfo {
    id: number | undefined | null;
    pricePerGallon: number;
    totalCost: number;
    gallons: number;
    aTripometer: number;
    bTripometer: number;
    totalTripometer: number
    timeRecorded: Date;
    city: string;
    state: string;
}
