package run.ccfish.android.pathfinding.test;

import android.graphics.Bitmap;
import android.graphics.Canvas;
import android.graphics.Color;
import android.graphics.Paint;
import android.graphics.Rect;
import android.os.Bundle;
import android.widget.ImageView;
import android.widget.TextView;

import androidx.appcompat.app.AppCompatActivity;

import java.text.MessageFormat;

import run.ccfish.android.pathfinding.PathFinding;

public class MainActivity extends AppCompatActivity {

    ImageView imgMap;
    TextView tvTime;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        imgMap = findViewById(R.id.img_map);
        tvTime = findViewById(R.id.tv_time);

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

        System.out.println("地图大小:"+grid[0].length + "x" + grid.length);
        System.out.println("版本号："+PathFinding.version());

        PathFinding.loadMap(grid);

        int startX = 0;
        int startY = 1;

        long t1 = System.nanoTime();
        int[] points = PathFinding.findPath(startX, startY, 15, 14);
        double d = (System.nanoTime() - t1)/1000./1000.;
        System.out.println("time:"+d+"ms");
        tvTime.setText(MessageFormat.format("{0}ms", d));

        int gridSize = dip2px(20);

        Bitmap bitmap = Bitmap.createBitmap(gridSize * grid[0].length, gridSize * grid.length, Bitmap.Config.ARGB_8888);
        Canvas canvas = new Canvas(bitmap);

        Paint paint = new Paint();
        paint.setAntiAlias(true);
        paint.setStyle(Paint.Style.FILL);
        paint.setColor(Color.WHITE);

        canvas.drawRect(new Rect(0, 0, bitmap.getWidth(), bitmap.getHeight()), paint);
        
        paint.setColor(Color.parseColor("#ff40e0ec"));
        for(int x=0; x<grid[0].length; x++){
            for(int y=0; y<grid.length; y++){
                if(grid[y][x] != 0){
                    int rx = x*gridSize;
                    int ry = y*gridSize;
                    canvas.drawRect(new Rect(rx, ry, rx+gridSize, ry+gridSize), paint);
                }
            }
        }

        int rx = startX*gridSize;
        int ry = startY*gridSize;
        paint.setColor(Color.parseColor("#ffc04851"));
        canvas.drawRect(new Rect(rx, ry, rx+gridSize, ry+gridSize), paint);

        paint.setColor(Color.parseColor("#77c04851"));
        for(int i=0; i<points.length; i+=2){
            rx = points[i]*gridSize;
            ry = points[i+1]*gridSize;
            if(i == points.length-2){
                paint.setColor(Color.parseColor("#ffc04851"));
            }
            canvas.drawRect(new Rect(rx, ry, rx+gridSize, ry+gridSize), paint);
        }

        //绘制线段
        points = PathFinding.getLineSegment(startX, startY, 15, 14);
        paint.setColor(Color.parseColor("#55faff00"));
        for(int i=0; i<points.length; i+=2){
            rx = points[i]*gridSize;
            ry = points[i+1]*gridSize;
            canvas.drawRect(new Rect(rx, ry, rx+gridSize, ry+gridSize), paint);
        }

        imgMap.setImageBitmap(bitmap);
    }

    public int dip2px(float dpValue) {
        final float scale = getResources().getDisplayMetrics().density;
        return (int) (dpValue * scale + 0.5f);
    }
}