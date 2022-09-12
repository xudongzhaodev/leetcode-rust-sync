class Solution {
    public int candy(int[] ratings) {
        int size = ratings.length;
        if (size < 2) {
            return size;
        }
        int[] c = new int[size];
        Arrays.fill(c, 1);
        for (int i = 1; i < size; ++i) {
            if (ratings[i] > ratings[i - 1]) {
                c[i] = c[i - 1] + 1;
            }
        }

        for (int i = size - 1; i > 0; --i) {
            if (ratings[i] < ratings[i - 1]) {
                c[i - 1] = Math.max(c[i - 1], c[i] + 1);
            }
        }

        return Arrays.stream(c).sum();
    }
}
