import { PublicKey } from '@solana/web3.js';
import { TIME_SLICE_DURATIONS, PDA_SEEDS } from '../constants';

export function generateTimeSliceId(
  chainId: number,
  slotNumber: number,
  timestamp: number
): string {
  return `STIME-${chainId}-${slotNumber}-${timestamp}`;
}

export function calculateTimeSliceDuration(startTime: number, endTime: number): number {
  return endTime - startTime;
}

export function isValidTimeSliceDuration(duration: number): boolean {
  return (
    duration >= TIME_SLICE_DURATIONS.MINIMUM &&
    duration <= TIME_SLICE_DURATIONS.SUPER
  );
}

export function generatePdaAddress(
  seeds: Buffer[],
  programId: PublicKey
): Promise<[PublicKey, number]> {
  return PublicKey.findProgramAddress(seeds, programId);
}

export function formatTimeSliceId(id: string): string {
  const [prefix, chainId, slotNumber, timestamp] = id.split('-');
  return `${prefix}-${chainId}-${slotNumber}-${new Date(parseInt(timestamp)).toISOString()}`;
}

export function calculateRarityScore(
  timeSliceDuration: number,
  historicalEvents: number,
  specialEvents: number
): number {
  const baseScore = timeSliceDuration / TIME_SLICE_DURATIONS.MINIMUM;
  const eventMultiplier = 1 + (historicalEvents * 0.1) + (specialEvents * 0.5);
  return Math.floor(baseScore * eventMultiplier);
}

export function validateTimeRange(startTime: number, endTime: number): boolean {
  const now = Date.now();
  return (
    startTime > now &&
    endTime > startTime &&
    calculateTimeSliceDuration(startTime, endTime) <= TIME_SLICE_DURATIONS.SUPER
  );
}

export function formatSolAmount(amount: number): string {
  return `${amount.toFixed(9)} SOL`;
}

export function sleep(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms));
} 