allprojects {
    repositories {
        google()
        mavenCentral()
    }
}

val newBuildDir: Directory =
    rootProject.layout.buildDirectory
        .dir("../../build")
        .get()
rootProject.layout.buildDirectory.value(newBuildDir)

subprojects {
    val newSubprojectBuildDir: Directory = newBuildDir.dir(project.name)
    project.layout.buildDirectory.value(newSubprojectBuildDir)
}

subprojects {
    project.evaluationDependsOn(":app")
}

subprojects {
    project.configurations.all {
        resolutionStrategy {
            force("androidx.fragment:fragment:1.5.7")
            force("androidx.core:core-ktx:1.10.1")
            force("androidx.core:core:1.10.1")
            force("androidx.activity:activity:1.7.2")
            force("androidx.lifecycle:lifecycle-runtime:2.6.1")
            force("androidx.lifecycle:lifecycle-livedata:2.6.1")
            force("androidx.lifecycle:lifecycle-livedata-core:2.6.1")
            force("androidx.lifecycle:lifecycle-livedata-core-ktx:2.6.1")
            force("androidx.lifecycle:lifecycle-viewmodel:2.6.1")
            force("androidx.lifecycle:lifecycle-viewmodel-savedstate:2.6.1")
            force("androidx.lifecycle:lifecycle-process:2.6.1")
            force("androidx.window:window:1.0.0")
            force("androidx.window:window-java:1.0.0")
            force("androidx.annotation:annotation-experimental:1.3.0")
            force("androidx.exifinterface:exifinterface:1.3.6")
        }
    }
}

tasks.register<Delete>("clean") {
    delete(rootProject.layout.buildDirectory)
}
