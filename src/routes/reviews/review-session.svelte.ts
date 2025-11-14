class ReviewSession {
  public reviewIds: number[] = $state([]);

  public reset() {
    this.reviewIds = [];
  }
}

export const reviewSession = $state(new ReviewSession());
