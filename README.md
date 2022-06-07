Ray tracing in one weekend in Rust
===
A Rust implementation of Peter Shirley's [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html).

## How to run
On Windows: 
```
cargo run --release
```
The program will emit an image file (`image.png`).
# Image outputs
Images showing the progression of the book, chapter by chapter.

## 2 Output an Image
![](images/2.2.png)

## 4 Rays, a Simple Camera, and Background
![](images/4.2.png) 

## 5 Adding a sphere
![](images/5.2.png)

## 6 Surface Normals and Multiple Objects

### 6.1 Shading with Surface Normals
![](images/6.1.png)

### 6.7 Common Constants and Utility Functions
![](images/6.7.png)

## 7 Antialiasing
![](images/7.2.png)

![](images/7.2%20-%20comparison.png)

*Before and after antialiasing*

## 8 Diffuse Materials

### 8.1 A Simple Diffuse Material
![](images/8.2.png)

### 8.3 Using Gamma Correction for Accurate Color Intensity
![](images/8.3.png)

### 8.4 Fixing Shadow Acne
![](images/8.4.png)