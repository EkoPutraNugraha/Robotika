from controller import Robot

# Create the Robot instance.
robot = Robot()

# Get the time step of the current world.
timestep = int(robot.getBasicTimeStep())

# Initialize the proximity sensor
proximity0 = robot.getDevice('ps0')
proximity0.enable(timestep)

# Initialize the motors
left_motor = robot.getDevice('left wheel motor')
right_motor = robot.getDevice('right wheel motor')

# Set the motors to velocity control mode
left_motor.setPosition(float('inf'))  # Infinite position mode for velocity control
right_motor.setPosition(float('inf'))

# Set initial motor velocities (positive values move the robot forward)
left_motor.setVelocity(2.0)
right_motor.setVelocity(2.0)

# Main Loop:
# Perform simulation steps until Webots is stopping the controller or the proximity sensor value exceeds 80
while robot.step(timestep) != -1:
    # Read the sensor value
    val_ps0 = proximity0.getValue()

    print("Proximity Sensor 0: {}".format(val_ps0))

    # Check if the proximity value exceeds 80, if so, stop the robot
    if val_ps0 > 80:
        print("Proximity sensor value exceeded 80. Stopping the robot.")
        left_motor.setVelocity(0.0)  # Stop the left motor
        right_motor.setVelocity(0.0)  # Stop the right motor
        break  # Exit the loop to stop the robot
    
    # Otherwise, the robot continues to move forward
    pass

# Enter here exit cleanup code if necessary.
