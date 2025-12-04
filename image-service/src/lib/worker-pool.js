/**
 * Worker Pool Manager
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Manages a pool of worker threads for parallel image processing.
 * Distributes tasks across workers and handles results.
 */

import { Worker } from 'worker_threads';
import path from 'path';
import { fileURLToPath } from 'url';
import os from 'os';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

class WorkerPool {
  constructor(options = {}) {
    this.poolSize = options.poolSize || Math.max(2, os.cpus().length - 1);
    this.workers = [];
    this.taskQueue = [];
    this.taskCallbacks = new Map();
    this.taskIdCounter = 0;
    this.workerPath = path.join(__dirname, '../workers/image-processor.js');
    this.initialized = false;
  }

  /**
   * Initialize the worker pool
   */
  async initialize() {
    if (this.initialized) return;

    for (let i = 0; i < this.poolSize; i++) {
      const worker = this.createWorker();
      this.workers.push({ worker, busy: false });
    }

    this.initialized = true;
    console.log(`Worker pool initialized with ${this.poolSize} workers`);
  }

  /**
   * Create a new worker
   */
  createWorker() {
    const worker = new Worker(this.workerPath);

    worker.on('message', (message) => {
      const callback = this.taskCallbacks.get(message.taskId);
      if (callback) {
        this.taskCallbacks.delete(message.taskId);
        if (message.success) {
          callback.resolve(message.result);
        } else {
          callback.reject(new Error(message.error));
        }
      }

      // Mark worker as available and process next task
      const workerInfo = this.workers.find((w) => w.worker === worker);
      if (workerInfo) {
        workerInfo.busy = false;
        this.processQueue();
      }
    });

    worker.on('error', (error) => {
      console.error('Worker error:', error);
      // Restart the worker
      const index = this.workers.findIndex((w) => w.worker === worker);
      if (index !== -1) {
        this.workers[index] = { worker: this.createWorker(), busy: false };
      }
    });

    return worker;
  }

  /**
   * Execute a task in the worker pool
   */
  async executeTask(type, buffer, options = {}) {
    if (!this.initialized) {
      await this.initialize();
    }

    const taskId = ++this.taskIdCounter;

    return new Promise((resolve, reject) => {
      const task = {
        taskId,
        type,
        buffer: Array.from(buffer),
        options,
      };

      this.taskCallbacks.set(taskId, { resolve, reject });
      this.taskQueue.push(task);
      this.processQueue();
    });
  }

  /**
   * Process the task queue
   */
  processQueue() {
    if (this.taskQueue.length === 0) return;

    const availableWorker = this.workers.find((w) => !w.busy);
    if (!availableWorker) return;

    const task = this.taskQueue.shift();
    availableWorker.busy = true;
    availableWorker.worker.postMessage(task);
  }

  /**
   * Execute multiple tasks in parallel
   */
  async executeParallel(tasks) {
    return Promise.all(
      tasks.map((task) => this.executeTask(task.type, task.buffer, task.options))
    );
  }

  /**
   * Get pool statistics
   */
  getStats() {
    const busy = this.workers.filter((w) => w.busy).length;
    return {
      poolSize: this.poolSize,
      busyWorkers: busy,
      availableWorkers: this.poolSize - busy,
      queueLength: this.taskQueue.length,
      pendingTasks: this.taskCallbacks.size,
    };
  }

  /**
   * Shutdown the worker pool
   */
  async shutdown() {
    for (const { worker } of this.workers) {
      await worker.terminate();
    }
    this.workers = [];
    this.initialized = false;
    console.log('Worker pool shut down');
  }
}

// Export singleton instance
export const workerPool = new WorkerPool();
export default WorkerPool;
