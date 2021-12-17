package run.ccfish.android.pathfinding;

import java.util.ArrayList;
import java.util.List;

public class PathFinding {
    static {
        System.loadLibrary("android_pathfinding");
    }

    public static native String version();

    /**
     * 加载地图
     * @param grid 表格地图
     */
    public static native void loadMap(int[][] grid);

    /**
     * 加载地图
     * @param key 地图的key
     * @param grid 表格地图
     */
    public static native void loadMapForKey(String key, int[][] grid);

    /**
     * 查找路径
     * @param x1 起点x
     * @param y1 起点y
     * @param x2 终点x
     * @param y2 终点y
     * @return 返回所有点的坐标[x,y,x,y...]
     */
    public static native int[] findPath(int x1, int y1, int x2, int y2);

    /**
     * 获取两点之间的直线线段
     * @param x1 起点x
     * @param y1 起点y
     * @param x2 终点x
     * @param y2 终点y
     * @return 返回所有点的坐标[x,y,x,y...]
     */
    public static native int[] getLineSegment(int x1, int y1, int x2, int y2);

    /**
     * 查找路径
     * @param key 地图的key
     * @param x1 起点x
     * @param y1 起点y
     * @param x2 终点x
     * @param y2 终点y
     * @return 返回所有点的坐标[x,y,x,y...]
     */
    public static native int[] findPathInMap(String key, int x1, int y1, int x2, int y2);

    /**
     * 查找路径
     * @param pointsList List < [x1, y1, x2, y2] >
     * @return 返回所有点的坐标List < [x,y,x,y...] >
     */
    public static List<int[]> findPaths(List<int[]> pointsList){
        List<int[]> results = new ArrayList<>();
        for(int[] points : pointsList){
            results.add(findPath(points[0], points[1], points[2], points[3]));
        }
        return results;
    }

    /**
     * 查找路径
     * @param key 地图key
     * @param pointsList List < [x1, y1, x2, y2] >
     * @return 返回所有点的坐标List < [x,y,x,y...] >
     */
    public static List<int[]> findPathsInMap(String key, List<int[]> pointsList){
        List<int[]> results = new ArrayList<>();
        for(int[] points : pointsList){
            results.add(findPathInMap(key, points[0], points[1], points[2], points[3]));
        }
        return results;
    }
}
