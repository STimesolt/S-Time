import { Connection, PublicKey, Transaction } from '@solana/web3.js';
import { TimeAssetManager } from '../contracts/TimeAssetManager';
import { TimeSlice } from '../types';
import { generateTimeSliceId, validateTimeRange } from '../utils/helpers';
import { TIME_SLICE_DURATIONS } from '../constants';

export class TimeSliceService {
  constructor(
    private connection: Connection,
    private timeAssetManager: TimeAssetManager
  ) {}

  async createTimeSlice(
    owner: PublicKey,
    startTime: number,
    endTime: number,
    metadata: string
  ): Promise<Transaction> {
    if (!validateTimeRange(startTime, endTime)) {
      throw new Error('Invalid time range');
    }

    const transaction = new Transaction();
    const instruction = await this.timeAssetManager.createTimeSlice(
      owner,
      startTime,
      endTime,
      metadata
    );
    transaction.add(instruction);

    return transaction;
  }

  async transferTimeSlice(
    from: PublicKey,
    to: PublicKey,
    timeSliceId: string
  ): Promise<Transaction> {
    const transaction = new Transaction();
    const instruction = await this.timeAssetManager.transferTimeSlice(
      from,
      to,
      timeSliceId
    );
    transaction.add(instruction);

    return transaction;
  }

  async getTimeSlice(timeSliceId: string): Promise<TimeSlice> {
    return this.timeAssetManager.getTimeSlice(timeSliceId);
  }

  async getTimeSlicesByOwner(owner: PublicKey): Promise<TimeSlice[]> {
    // Implementation for fetching all time slices owned by an address
    // This would require additional program methods and filters
    return [];
  }

  async getTimeSlicesByTimeRange(
    startTime: number,
    endTime: number
  ): Promise<TimeSlice[]> {
    // Implementation for fetching all time slices within a time range
    // This would require additional program methods and filters
    return [];
  }

  async calculateTimeSliceValue(timeSlice: TimeSlice): Promise<number> {
    const duration = timeSlice.endTime - timeSlice.startTime;
    const baseValue = duration / TIME_SLICE_DURATIONS.MINIMUM;
    const rarityMultiplier = 1 + (timeSlice.rarityScore * 0.1);
    return baseValue * rarityMultiplier;
  }
} 