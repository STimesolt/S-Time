import { PublicKey } from '@solana/web3.js';

export interface TimeSlice {
  id: string;
  startTime: number;
  endTime: number;
  owner: PublicKey;
  mintInfo: {
    blockHeight: number;
    transactionHash: string;
  };
  permissionLevel: number;
  rarityScore: number;
  status: number;
  metadata: string;
}

export interface Order {
  id: string;
  timeSliceId: string;
  seller: PublicKey;
  price: number;
  orderType: OrderType;
  status: OrderStatus;
  createdAt: number;
}

export interface Reservation {
  id: string;
  timeSliceId: string;
  user: PublicKey;
  startTime: number;
  endTime: number;
  status: ReservationStatus;
  deposit: number;
  createdAt: number;
}

export interface Permission {
  id: string;
  timeSliceId: string;
  user: PublicKey;
  permissionLevel: PermissionLevel;
  resourceType: ResourceType;
  priority: number;
  status: PermissionStatus;
  createdAt: number;
}

export enum OrderType {
  LIMIT = 0,
  MARKET = 1,
  STOP = 2
}

export enum OrderStatus {
  PENDING = 0,
  EXECUTED = 1,
  CANCELLED = 2,
  EXPIRED = 3
}

export enum ReservationStatus {
  PENDING = 0,
  CONFIRMED = 1,
  CANCELLED = 2,
  COMPLETED = 3
}

export enum PermissionLevel {
  NONE = 0,
  READ = 1,
  WRITE = 2,
  ADMIN = 3
}

export enum ResourceType {
  COMPUTE = 0,
  STORAGE = 1,
  NETWORK = 2,
  ALL = 3
}

export enum PermissionStatus {
  INACTIVE = 0,
  ACTIVE = 1,
  REVOKED = 2
} 