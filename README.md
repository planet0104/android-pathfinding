# android-pathfinding

Android pathfinding

[![](https://jitpack.io/v/planet0104/android-pathfinding.svg)](https://jitpack.io/#planet0104/android-pathfinding)

```gradle
dependencies {
        implementation 'com.github.planet0104:android-pathfinding:1.0.0'
}
```


```java
int[][] grid = new int[][]{
        {1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1},
        {0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,1},
        {0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,1},
        {1,0,0,1,0,0,1,1,1,1,1,1,1,0,0,1},
        {1,0,0,1,0,0,0,0,0,0,0,0,1,0,0,1},
        {1,0,0,1,0,0,0,0,0,0,0,0,1,0,0,1},
        {1,0,0,1,1,1,1,1,1,1,0,0,1,1,1,1},
        {1,0,0,1,0,0,0,0,0,1,0,0,0,0,0,1},
        {1,0,0,1,0,0,0,0,0,1,0,0,0,0,0,1},
        {1,0,0,1,0,0,1,0,0,1,1,1,1,0,0,1},
        {1,0,0,0,0,0,1,0,0,1,0,0,0,0,0,1},
        {1,0,0,0,0,0,1,0,0,1,0,0,0,0,0,1},
        {1,0,0,1,1,1,1,0,0,1,0,0,1,0,0,1},
        {1,0,0,1,0,0,0,0,0,0,0,0,1,0,0,0},
        {1,0,0,1,0,0,0,0,0,0,0,0,1,0,0,0},
        {1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1},
};

PathFinding.loadMap(grid);

int[] points = PathFinding.findPath(0, 1, 15, 14);
```


![Image](https://github.com/planet0104/android-pathfinding/blob/main/screenshot.png)
