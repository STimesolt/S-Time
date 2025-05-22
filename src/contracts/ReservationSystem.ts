import { PublicKey, SystemProgram, TransactionInstruction } from '@solana/web3.js';
import { Program } from '@project-serum/anchor';

export interface Reservation {
  id: string;
  timeSliceId: string;
  user: PublicKey;
  startTime: number;
  endTime: number;
  status: number;
  deposit: number;
  createdAt: number;
}

export class ReservationSystem {
  constructor(
    private program: Program,
    private reservationProgramId: PublicKey
  ) {}

  async createReservation(
    user: PublicKey,
    timeSliceId: string,
    startTime: number,
    endTime: number,
    deposit: number
  ): Promise<TransactionInstruction> {
    const [reservationPda] = await PublicKey.findProgramAddress(
      [
        Buffer.from('reservation'),
        user.toBuffer(),
        Buffer.from(timeSliceId),
      ],
      this.reservationProgramId
    );

    return this.program.methods
      .createReservation(timeSliceId, startTime, endTime, deposit)
      .accounts({
        reservation: reservationPda,
        user,
        systemProgram: SystemProgram.programId,
      })
      .instruction();
  }

  async cancelReservation(
    user: PublicKey,
    reservationId: string
  ): Promise<TransactionInstruction> {
    const [reservationPda] = await PublicKey.findProgramAddress(
      [
        Buffer.from('reservation'),
        Buffer.from(reservationId),
      ],
      this.reservationProgramId
    );

    return this.program.methods
      .cancelReservation()
      .accounts({
        reservation: reservationPda,
        user,
        systemProgram: SystemProgram.programId,
      })
      .instruction();
  }

  async getReservation(reservationId: string): Promise<Reservation> {
    const [reservationPda] = await PublicKey.findProgramAddress(
      [
        Buffer.from('reservation'),
        Buffer.from(reservationId),
      ],
      this.reservationProgramId
    );

    const reservationAccount = await this.program.account.reservation.fetch(reservationPda);
    return reservationAccount as unknown as Reservation;
  }
} 